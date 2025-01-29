use std::error::Error;
use std::future::Future;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use std::{fmt, io, vec};

use tokio::task::JoinHandle;
use tower_service::Service;
use tracing::debug_span;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Name {
    host: Box<str>,
}

impl Name {
    pub fn new(host: Box<str>) -> Name {
        Name { host }
    }

    pub fn as_str(&self) -> &str {
        &self.host
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.host, f)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.host, f)
    }
}

impl FromStr for Name {
    type Err = InvalidNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Name::new(s.into()))
    }
}

#[derive(Debug)]
pub struct InvalidNameError(());

impl fmt::Display for InvalidNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Not a valid domain name")
    }
}

impl Error for InvalidNameError {}

pub struct Addrs {
    inner: SocketAddrs,
}

impl Iterator for Addrs {
    type Item = SocketAddr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl fmt::Debug for Addrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Addrs").finish()
    }
}

pub struct SocketAddrs {
    iter: vec::IntoIter<SocketAddr>,
}

impl SocketAddrs {
    pub fn new(addrs: Vec<SocketAddr>) -> SocketAddrs {
        SocketAddrs {
            iter: addrs.into_iter(),
        }
    }

    pub fn try_parse(host: &str, port: u16) -> Option<SocketAddrs> {
        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(addr, port);
            return Some(SocketAddrs {
                iter: vec![SocketAddr::V4(addr)].into_iter(),
            });
        }
        if let Ok(addr) = host.parse::<Ipv6Addr>() {
            let addr = SocketAddrV6::new(addr, port, 0, 0);
            return Some(SocketAddrs {
                iter: vec![SocketAddr::V6(addr)].into_iter(),
            });
        }
        None
    }

    pub fn split_by_preference(
        self,
        local_addr_ipv4: Option<Ipv4Addr>,
        local_addr_ipv6: Option<Ipv6Addr>,
    ) -> (SocketAddrs, SocketAddrs) {
        match (local_addr_ipv4, local_addr_ipv6) {
            (Some(_), None) => (
                self.filter(SocketAddr::is_ipv4),
                SocketAddrs::new(Vec::new()),
            ),
            (None, Some(_)) => (
                self.filter(SocketAddr::is_ipv6),
                SocketAddrs::new(Vec::new()),
            ),
            _ => {
                let preferring_v6 = self
                    .iter
                    .as_slice()
                    .first()
                    .map(SocketAddr::is_ipv6)
                    .unwrap_or(false);

                let (preferred, fallback) = self
                    .iter
                    .partition::<Vec<_>, _>(|addr| addr.is_ipv6() == preferring_v6);

                (SocketAddrs::new(preferred), SocketAddrs::new(fallback))
            }
        }
    }

    #[inline]
    fn filter(self, predicate: impl FnMut(&SocketAddr) -> bool) -> SocketAddrs {
        SocketAddrs::new(self.iter.filter(predicate).collect())
    }

    pub fn is_empty(&self) -> bool {
        self.iter.as_slice().is_empty()
    }

    pub fn len(&self) -> usize {
        self.iter.as_slice().len()
    }
}

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Clone)]
pub struct Resolver {
    _priv: (),
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver { _priv: () }
    }
}

impl Service<Name> for Resolver {
    type Response = Addrs;
    type Error = io::Error;
    type Future = ResolverFuture;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, name: Name) -> Self::Future {
        let span = debug_span!("resolve", host = %name.host);
        let blocking = tokio::task::spawn_blocking(move || {
            let _enter = span.enter();
            (&*name.host, 0)
                .to_socket_addrs()
                .map(|i| SocketAddrs { iter: i })
        });

        ResolverFuture(blocking)
    }
}

impl fmt::Debug for Resolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Resolver")
    }
}

struct ResolverFuture(JoinHandle<Result<SocketAddrs, io::Error>>);

impl Future for ResolverFuture {
    type Output = Result<Addrs, io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx).map(|res| match res {
            Ok(Ok(addrs)) => Ok(Addrs { inner: addrs }),
            Ok(Err(e)) => Err(e),
            Err(join_err) => {
                if join_err.is_cancelled() {
                    Err(io::Error::new(io::ErrorKind::Interrupted, join_err))
                } else {
                    panic!("resolver background task failed: {:?}", join_err)
                }
            }
        })
    }
}

impl fmt::Debug for ResolverFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("ResolverFuture")
    }
}

impl Drop for ResolverFuture {
    fn drop(&mut self) {
        self.0.abort();
    }
}
