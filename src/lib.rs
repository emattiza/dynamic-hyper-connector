use std::{error::Error, str::FromStr, task::Poll, sync::Arc};

use futures_util::{future::BoxFuture, Future, AsyncRead};
use hyper::{client::{connect::{Connection, Connected}}, service::Service, Body, Client, Uri};
use tokio::net::{UnixStream, TcpStream};


// impl Connection for MultiConnection {
//     fn connected(&self) -> hyper::client::connect::Connected {
//         Connected::new().into()
//     }
// }

impl Future for MultiConnection {
    type Output = Result<(), std::io::Error>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        self.poll_ready(cx)
    }
}

impl MultiConnection {
    async fn new(url: &str) {
        MultiConnection::Unix(Arc::new(UnixStream::connect(path)))
    }
}

#[derive(Clone)]
pub(crate) enum MultiConnection {
    Unix(Arc<UnixStream>),
    Http(Arc<TcpStream>)
}

impl Service<Uri> for MultiConnection {
    type Response = ();

    type Error = std::io::Error;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        match req.scheme_str() {
            Some("http") => unimplemented!(),
            Some("unix") => unimplemented!(),
            Some(_) => unimplemented!(),
            None => unimplemented!()
        }
    }
}

fn generic_client(url: &str) -> Result<Client<MultiConnection, Body>, Box<dyn Error>> {
    match Uri::from_str(url) {
        Ok(uri) => Ok(Client::builder().build(MultiConnection::new(url).call(uri))),
        Err(err) => Err(Box::new(err)),
    }
}