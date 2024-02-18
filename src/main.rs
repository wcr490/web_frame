use std::borrow::BorrowMut;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use hyper::Response;
use std::fs;

use frame::{conf_to_iter, exe_generate};
use frame::{hyper_manager::server::*, route_manager::route::*, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut r = Route::new();
    r.insert("/example".to_string());
    // if let Some((k, v)) = r.into_iter().find(|(k, _)| k.eq("/example")) {
    //     v.call();
    // }
    exe_generate!(example_two, "ex".to_string(), {
        println!("example_two");
        Ok::<_, hyper::Error>(Response::new(full(
            fs::read_to_string("hello.html").unwrap(),
        )))
    });
    example_two.call();
    let mut route = Route::new();
    route.insert("/ok".to_string());
    let mut conf = Config::with_route(route);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr, conf).await
}
