use super::super::*;

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::{Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::borrow::Borrow;
use std::fs;
use std::net::SocketAddr;
use std::ops::{Deref, Index};
use tokio::net::TcpListener;

use futures::lock::Mutex;
use std::sync::Arc;

pub async fn run_server(
    addr: SocketAddr,
    conf: Arc<Mutex<Config>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    loop {
        let conf_clone = Arc::clone(&conf);
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| {
                        let conf_clone = Arc::clone(&conf_clone);
                        async move {
                            let guard = conf_clone.lock().await;
                            let config = (*guard).clone();
                            let exe = req_to_exe(&config, req).await;
                            let fut = async move { exe.unwrap().0.call() };
                            fut.await
                        }
                    }),
                )
                .await
            {
                println!("Err: {}", e);
            }
        });
    }
}
async fn req_to_exe(conf: &Config, req: Request<hyper::body::Incoming>) -> Option<&Cb> {
    let path = req.uri().path().to_string();
    if conf.exec.contains_key(&path) && conf.method.contains_key(&path) {
        conf.exec.get(&path).clone()
    } else {
        None
    }
}
pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
