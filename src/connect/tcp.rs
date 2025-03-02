use std::future::Future;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::future::Either;
use http::uri::{Scheme, Uri};
use hyper_util::rt::TokioIo;
use ipnet::IpNet;
use rand::Rng;
use tokio::net::{TcpSocket, TcpStream};
use tokio::time::Sleep;
use tower_service::Service;

use super::dns::{self, resolve, Resolve, Resolver};
use super::error::{DnsError, Error, InvalidUriError, TcpError};

#[derive(Clone)]
struct Config {
    connect_timeout: Option<Duration>,
    happy_eyeballs_timeout: Option<Duration>,
    local_address_ipv4: Option<Ipv4Addr>,
    local_address_ipv6: Option<Ipv6Addr>,
    nodelay: bool,
}

#[derive(Clone)]
pub struct TcpConnector<R = Resolver>
where
    R: Resolve,
{
    config: Arc<Config>,
    resolver: R,
}

impl TcpConnector {
    pub fn new() -> Self {
        Self::new_with_resolver(Resolver::new())
    }
}

impl<R> TcpConnector<R>
where
    R: Resolve,
{
    pub fn new_with_resolver(resolver: R) -> TcpConnector<R> {
        Self {
            config: Arc::new(Config {
                connect_timeout: None,
                happy_eyeballs_timeout: Some(Duration::from_millis(300)),
                local_address_ipv4: None,
                local_address_ipv6: None,
                nodelay: false,
            }),
            resolver,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_connect_timeout(&mut self, dur: Option<Duration>) {
        self.config_mut().connect_timeout = dur;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_happy_eyeballs_timeout(&mut self, dur: Option<Duration>) {
        self.config_mut().happy_eyeballs_timeout = dur;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_local_address(&mut self, addr: Option<IpAddr>) {
        let (v4, v6) = match addr {
            Some(IpAddr::V4(a)) => (Some(a), None),
            Some(IpAddr::V6(a)) => (None, Some(a)),
            _ => (None, None),
        };

        let cfg = self.config_mut();

        cfg.local_address_ipv4 = v4;
        cfg.local_address_ipv6 = v6;
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_local_addresses(&mut self, addr_ipv4: Ipv4Addr, addr_ipv6: Ipv6Addr) {
        let cfg = self.config_mut();

        cfg.local_address_ipv4 = Some(addr_ipv4);
        cfg.local_address_ipv6 = Some(addr_ipv6);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn set_nodelay(&mut self, nodelay: bool) {
        self.config_mut().nodelay = nodelay;
    }

    pub fn assign_local_address_from_cidr(&mut self, cidr: Option<IpNet>) {
        if let Some(cidr) = cidr {
            let mut rng = rand::rng();

            let addr = match cidr {
                IpNet::V4(net) => {
                    let prefix_len = net.prefix_len();
                    let host_len = 32u8 - prefix_len;

                    let network_bits = net.network().to_bits();
                    let rand_host_bits = if prefix_len < 31 {
                        // exclude network address and broadcast address
                        1 + rng.random_range(0..(1u32 << host_len) - 2)
                    } else {
                        // no need to exclude
                        rng.random_range(0..(1u32 << host_len))
                    };

                    IpAddr::V4(Ipv4Addr::from_bits(network_bits | rand_host_bits))
                }
                IpNet::V6(net) => {
                    let prefix_len = net.prefix_len();
                    let host_len = 128u8 - prefix_len;

                    let network_bits = net.network().to_bits();
                    let rand_host_bits = rng.random_range(0..(1u128 << host_len));

                    IpAddr::V6(Ipv6Addr::from_bits(network_bits | rand_host_bits))
                }
            };

            tracing::trace!("assigning local address: {:?}", addr);

            self.set_local_address(Some(addr))
        }
    }

    fn config_mut(&mut self) -> &mut Config {
        Arc::make_mut(&mut self.config)
    }
}

impl<R> Service<Uri> for TcpConnector<R>
where
    R: Resolve + Clone + Send + Sync + 'static,
    R::Future: Send,
{
    type Response = TokioIo<TcpStream>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        futures_util::ready!(self.resolver.poll_ready(cx))
            .map_err(|e| DnsError(io::Error::new(io::ErrorKind::Other, e)))?;
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, dst: Uri) -> Self::Future {
        let mut self_ = self.clone();
        Box::pin(async move {
            let config = &self_.config;

            let (host, port) = get_host_port(&dst)?;
            let host = host.trim_start_matches('[').trim_end_matches(']');

            let addrs = if let Some(addrs) = dns::SocketAddrs::try_parse(host, port) {
                addrs
            } else {
                let addrs = resolve(&mut self_.resolver, dns::Name::new(host.into()))
                    .await
                    .map_err(|e| DnsError(io::Error::new(io::ErrorKind::Other, e)))?;

                let addrs = addrs
                    .map(|mut addr| {
                        set_port(&mut addr, port, dst.port().is_some());
                        addr
                    })
                    .collect();

                dns::SocketAddrs::new(addrs)
            };

            let c = ConnectingTcp::new(addrs, config);

            let sock = c.connect().await?;

            if let Err(e) = sock.set_nodelay(config.nodelay) {
                tracing::warn!("tcp set_nodelay error: {:?}", e)
            }

            Ok(TokioIo::new(sock))
        })
    }
}

struct ConnectingTcp<'a> {
    preferred: ConnectingTcpRemote,
    fallback: Option<ConnectingTcpFallback>,
    config: &'a Config,
}

impl<'a> ConnectingTcp<'a> {
    fn new(remote_addrs: dns::SocketAddrs, config: &'a Config) -> Self {
        if let Some(fallback_timeout) = config.happy_eyeballs_timeout {
            let (preferred_addrs, fallback_addrs) = remote_addrs
                .split_by_preference(config.local_address_ipv4, config.local_address_ipv6);
            if fallback_addrs.is_empty() {
                return ConnectingTcp {
                    preferred: ConnectingTcpRemote::new(preferred_addrs, config.connect_timeout),
                    fallback: None,
                    config,
                };
            }

            ConnectingTcp {
                preferred: ConnectingTcpRemote::new(preferred_addrs, config.connect_timeout),
                fallback: Some(ConnectingTcpFallback {
                    delay: tokio::time::sleep(fallback_timeout),
                    remote: ConnectingTcpRemote::new(fallback_addrs, config.connect_timeout),
                }),
                config,
            }
        } else {
            ConnectingTcp {
                preferred: ConnectingTcpRemote::new(remote_addrs, config.connect_timeout),
                fallback: None,
                config,
            }
        }
    }
}

impl ConnectingTcp<'_> {
    async fn connect(mut self) -> Result<TcpStream, TcpError> {
        match self.fallback {
            None => self.preferred.connect(self.config).await,
            Some(mut fallback) => {
                let preferred_fut = self.preferred.connect(self.config);
                futures_util::pin_mut!(preferred_fut);

                let fallback_fut = fallback.remote.connect(self.config);
                futures_util::pin_mut!(fallback_fut);

                let fallback_delay = fallback.delay;
                futures_util::pin_mut!(fallback_delay);

                let (result, future) =
                    match futures_util::future::select(preferred_fut, fallback_delay).await {
                        Either::Left((result, _fallback_delay)) => {
                            (result, Either::Right(fallback_fut))
                        }
                        Either::Right(((), preferred_fut)) => {
                            futures_util::future::select(preferred_fut, fallback_fut)
                                .await
                                .factor_first()
                        }
                    };

                if result.is_err() {
                    // fallback to fallback_fut if error
                    future.await
                } else {
                    result
                }
            }
        }
    }
}

struct ConnectingTcpFallback {
    delay: Sleep,
    remote: ConnectingTcpRemote,
}

struct ConnectingTcpRemote {
    addrs: dns::SocketAddrs,
    connect_timeout: Option<Duration>,
}

impl ConnectingTcpRemote {
    fn new(addrs: dns::SocketAddrs, connect_timeout: Option<Duration>) -> Self {
        let connect_timeout = connect_timeout.and_then(|t| t.checked_div(addrs.len() as u32));

        Self {
            addrs,
            connect_timeout,
        }
    }

    async fn connect(&mut self, config: &Config) -> Result<TcpStream, TcpError> {
        let mut err = None;
        for addr in &mut self.addrs {
            tracing::debug!("connecting to {}", addr);
            match connect(&addr, config, self.connect_timeout)?.await {
                Ok(tcp) => {
                    tracing::debug!("connected to {}", addr);
                    return Ok(tcp);
                }
                Err(e) => {
                    tracing::trace!("connect error for {}: {:?}", addr, e);
                    err = Some(e);
                }
            }
        }

        match err {
            Some(e) => Err(e),
            None => Err(TcpError::from(io::Error::new(
                io::ErrorKind::NotConnected,
                "Network unreachable",
            ))),
        }
    }
}

fn connect(
    addr: &SocketAddr,
    config: &Config,
    connect_timeout: Option<Duration>,
) -> Result<impl Future<Output = Result<TcpStream, TcpError>>, TcpError> {
    let socket = match addr {
        SocketAddr::V4(_) => TcpSocket::new_v4().map_err(TcpError)?,
        SocketAddr::V6(_) => TcpSocket::new_v6().map_err(TcpError)?,
    };

    match (addr, &config.local_address_ipv4, &config.local_address_ipv6) {
        (SocketAddr::V4(_), Some(addr), _) => {
            socket
                .bind(SocketAddr::new((*addr).into(), 0))
                .map_err(TcpError)?;
        }
        (SocketAddr::V6(_), _, Some(addr)) => {
            socket
                .bind(SocketAddr::new((*addr).into(), 0))
                .map_err(TcpError)?;
        }
        _ => {
            if cfg!(windows) {
                // Windows requires a socket be bound before calling connect
                let any: SocketAddr = match *addr {
                    SocketAddr::V4(_) => ([0, 0, 0, 0], 0).into(),
                    SocketAddr::V6(_) => ([0, 0, 0, 0, 0, 0, 0, 0], 0).into(),
                };
                socket.bind(any).map_err(TcpError)?;
            }
        }
    }

    let connect = socket.connect(*addr);
    Ok(async move {
        match connect_timeout {
            Some(dur) => match tokio::time::timeout(dur, connect).await {
                Ok(Ok(s)) => Ok(s),
                Ok(Err(e)) => Err(e),
                Err(e) => Err(io::Error::new(io::ErrorKind::TimedOut, e)),
            },
            None => connect.await,
        }
        .map_err(TcpError)
    })
}

fn get_host_port(dst: &Uri) -> Result<(&str, u16), InvalidUriError> {
    tracing::trace!(
        "Http::connect; scheme={:?}, host={:?}, port={:?}",
        dst.scheme(),
        dst.host(),
        dst.port(),
    );

    let host = match dst.host() {
        Some(s) => s,
        None => {
            return Err(InvalidUriError::from(io::Error::new(
                io::ErrorKind::InvalidInput,
                "host is missing",
            )));
        }
    };

    let port = match dst.port() {
        Some(port) => port.as_u16(),
        None => {
            if dst.scheme() == Some(&Scheme::HTTPS) {
                443
            } else {
                80
            }
        }
    };

    Ok((host, port))
}

fn set_port(addr: &mut SocketAddr, host_port: u16, explicit: bool) {
    if explicit || addr.port() == 0 {
        addr.set_port(host_port)
    };
}
