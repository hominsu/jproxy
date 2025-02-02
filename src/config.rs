use std::future::{Future, IntoFuture};
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;

use config::{ConfigError, File};
use glob::glob;
use ipnet::IpNet;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Debug mode
    ///
    /// When enabled, the server will run in debug mode, providing more verbose logging.
    pub debug: bool,

    /// The address and port to bind the server to.
    ///
    /// This should be in the format of `address:port`. e.g.
    /// - `127.0.0.1:3000` binds to localhost on port 3000.
    /// - `0.0.0.0:3000` binds to all network interfaces on port 3000.
    pub bind: SocketAddr,

    /// Concurrent connections
    ///
    /// Specifies the limit of concurrent connections that the server can handle simultaneously.
    pub concurrent: u32,

    pub connect_timeout: Option<Duration>,

    pub cidr: Option<IpNet>,

    pub fallback: Option<IpAddr>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            bind: "0.0.0.0:3000".parse().unwrap(),
            concurrent: 1024,
            connect_timeout: Some(Duration::from_secs(10)),
            cidr: None,
            fallback: None,
        }
    }
}

impl Config {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let pattern = Path::new(path).join("*");
        config::Config::builder()
            .add_source(
                glob(pattern.to_str().unwrap())
                    .unwrap()
                    .filter_map(|entry| match entry {
                        Ok(path) => match path.extension() {
                            Some(ext)
                                if matches!(
                                    ext.to_str().unwrap(),
                                    "toml" | "json" | "yaml" | "ini" | "ron" | "json5"
                                ) =>
                            {
                                Some(File::from(path))
                            }
                            _ => None,
                        },
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>(),
            )
            .build()?
            .try_deserialize()
    }
}
pub fn manager(path: &str) -> Manager {
    let config = Config::new(path).unwrap();
    Manager {
        config: Arc::new(RwLock::new(config)),
        path: path.to_string(),
    }
}

pub struct Manager {
    config: Arc<RwLock<Config>>,
    path: String,
}

impl Manager {
    pub fn with_watcher<F>(&self, signal: F) -> WithWatcher<F>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        WithWatcher {
            config: self.config.clone(),
            path: self.path.clone(),
            signal,
        }
    }

    #[allow(dead_code)]
    pub fn config(&self) -> Arc<RwLock<Config>> {
        self.config.clone()
    }
}

impl IntoFuture for Manager {
    type Output = io::Result<()>;
    type IntoFuture = private::ManagerFuture;

    fn into_future(self) -> Self::IntoFuture {
        self.with_watcher(std::future::pending()).into_future()
    }
}

pub struct WithWatcher<F> {
    config: Arc<RwLock<Config>>,
    path: String,
    signal: F,
}

impl<F> WithWatcher<F> {
    #[allow(dead_code)]
    pub fn config(&self) -> Arc<RwLock<Config>> {
        self.config.clone()
    }
}

impl<F> IntoFuture for WithWatcher<F>
where
    F: Future<Output = ()> + Send + 'static,
{
    type Output = io::Result<()>;
    type IntoFuture = private::ManagerFuture;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            config,
            path,
            signal,
        } = self;

        private::ManagerFuture(Box::pin(async move {
            let (tx, rx) = mpsc::channel();

            let mut watcher: RecommendedWatcher = Watcher::new(
                tx,
                notify::Config::default().with_poll_interval(Duration::from_secs(2)),
            )
            .unwrap();

            watcher
                .watch(Path::new(path.as_str()), RecursiveMode::NonRecursive)
                .unwrap();

            let config_clone = config.clone();
            let path_clone = path.clone();
            let task = tokio::task::spawn_blocking(move || loop {
                match rx.recv() {
                    Ok(Ok(Event {
                        kind: notify::EventKind::Modify(_),
                        ..
                    })) => {
                        *config_clone.write().unwrap() = Config::new(path_clone.as_str()).unwrap();
                    }
                    Err(_) => break,
                    _ => {}
                }
            });

            tokio::pin!(signal, task);

            tokio::select! {
                _ = &mut signal => {
                    tracing::trace!("received graceful shutdown signal. Stopping watcher");
                    drop(watcher);
                    let _ = task.await;
                    Ok(())
                },
                res = &mut task => {
                    match res {
                        Ok(_) => Ok(()),
                        Err(err) => Err(io::Error::new(io::ErrorKind::Other, format!("watcher task failed: {err}"))),
                    }
                },
            }
        }))
    }
}

mod private {
    use std::{
        boxed::Box,
        future::Future,
        io,
        pin::Pin,
        task::{Context, Poll},
    };

    pub struct ManagerFuture(pub Pin<Box<dyn Future<Output = io::Result<()>> + Send>>);

    impl Future for ManagerFuture {
        type Output = io::Result<()>;

        #[inline]
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            self.0.as_mut().poll(cx)
        }
    }
}
