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

pub async fn run_server(
    addr: SocketAddr,
    config: Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|req| async {
                        Ok::<_, hyper::Error>(Response::new(full(
                            fs::read_to_string("hello.html").unwrap(),
                        )))
                    }),
                )
                .await
            {
                println!("Err: {}", e);
            }
        });
    }
}

fn req_to_exe(mut conf: Config, req: Request<hyper::body::Incoming>) -> Option<Exe> {
    let path = req.uri().path();
    if path_contains(&mut conf, &req) {
        conf.exec.remove(path)
    } else {
        None
    }
}
fn path_contains(conf: &mut Config, req: &Request<hyper::body::Incoming>) -> bool {
    let method = conf.method.get(req.uri().path());
    let exe = conf.exec.remove(req.uri().path());
    if !(method.is_none() || exe.is_none()) {
        true
    } else {
        false
    }
}
pub(crate) fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
