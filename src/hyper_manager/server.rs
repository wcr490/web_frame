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
    if let (method, path) = (req.method(), req.uri().path()) {}
    Ok(Response::new(full(
        fs::read_to_string("hello.html").unwrap(),
    )))
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
fn search_with_req(mut conf: Config, req: Request<hyper::body::Incoming>) -> Option<Exe> {
    let method = conf.method.get(req.uri().path());
    let exe = conf.exec.remove(req.uri().path());
    if !(method.is_none() || exe.is_none()) {
        exe
    } else {
        None
    }
}
