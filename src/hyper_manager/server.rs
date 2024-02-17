use super::*;

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::{Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::fs;
use std::io::Read;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn build(
    addr: SocketAddr,
    config: Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(home))
                .await
            {
                println!("Err: {}", e);
            }
        });
    }
}

async fn home(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let mut resp = Response::new(full(fs::read_to_string("hello.html").unwrap()));
    Ok(resp)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
