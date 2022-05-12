use std::{net::TcpStream, pin::Pin, str::FromStr, sync::Arc};

use anyhow::{anyhow, Error};
use futures_util::Future;
use hyper::{
    client::{connect::Connection, HttpConnector},
    service::Service,
    Body, Client, Uri,
};
use hyper_unix_connector::{UnixClient, UDS};
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Clone)]
pub enum MultiConnector {
    Http(HttpConnector),
    Unix(UnixClient),
}

#[derive(Clone)]
pub enum BaseConnection {
    Unix(Arc<UDS>),
    Http(Arc<TcpStream>),
}

impl Connection for BaseConnection {
    fn connected(&self) -> hyper::client::connect::Connected {
        todo!()
    }
}

impl AsyncRead for BaseConnection {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        todo!()
    }
}

impl AsyncWrite for BaseConnection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        todo!()
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        todo!()
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        todo!()
    }
}

trait AsyncRW: AsyncRead + AsyncWrite + Connection {}
impl Service<Uri> for MultiConnector {
    type Response = BaseConnection;

    type Error = Error;

    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        todo!()
    }
}

fn generic_client(url: &str) -> Result<Client<MultiConnector, Body>, anyhow::Error> {
    Uri::from_str(url)?
        .scheme_str()
        .filter(|&x| x.eq("http") || x.eq("unix") || x.eq("https"))
        .map::<MultiConnector, _>(|scheme: &str| -> MultiConnector {
            if let "http" | "https" = scheme {
                MultiConnector::Http(HttpConnector::new())
            } else {
                MultiConnector::Unix(UnixClient)
            }
        })
        .ok_or(anyhow!("Invalid Scheme"))
        .map(|conn| Client::builder().build(conn))
}

// Individual Builds
fn build_unix() -> Client<MultiConnector, Body> {
    Client::builder().build(MultiConnector::Unix(UnixClient))
}

fn build_http() -> Client<MultiConnector, Body> {
    Client::builder().build(MultiConnector::Http(HttpConnector::new()))
}
