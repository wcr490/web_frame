use futures::lock::Mutex;
use std::borrow::BorrowMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use hyper::Response;
use std::fs;

use frame::exe_generate;
use frame::{hyper_manager::server::*, route_manager::route::*, Config};
exe_generate!(Kk, "/exam".to_string(), {
    println!("Kk success");
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/hello.html").unwrap(),
    )))
});

exe_generate!(Gg, "/exam/gg".to_string(), {
    println!("Gg success");
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/hello.html").unwrap(),
    )))
});
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut route = Route::new();

    route.insert_path("/exam".to_string());
    route.insert_path("/exam/gg".to_string());

    route.insert_exe(Box::new(Kk), "/exam".to_string());
    route.insert_exe(Box::new(Gg), "/exam/gg".to_string());

    let conf = Config::with_route(route);
    run_server(addr, conf).await
}
