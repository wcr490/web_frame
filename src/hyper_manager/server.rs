use super::*;

/// main function to boot the server provided by Hyper
// please put your attention on the the closure in service_fn()
pub async fn run_server(
    addr: SocketAddr,
    conf: Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    // wrap the conf by Arc and Mutex to guarantee thread safety
    let conf = Arc::new(Mutex::new(conf));
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
                            let exe = req_to_exe(&guard, req).await;
                            let fut = async move {
                                match exe {
                                    Some(exist) => exist.0.call(),
                                    None => DefaultCallback.call(),
                                }
                            };
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
/// convert request into Cb which includes exe
///
/// # Parameter
/// * `conf`
/// * `req` - collection contains information
///
/// # Return
/// * Option<&Cb>
async fn req_to_exe(conf: &Config, req: Request<hyper::body::Incoming>) -> Option<&Cb> {
    let path = req.uri().path().to_string();
    println!("current page: {}", path);
    if conf.exec.contains_key(&path) {
        conf.exec.get(&path).clone()
    } else {
        None
    }
}
/// convert String into BoxBody which can be the response
pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
