use std::borrow::BorrowMut;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use hyper::Response;
use std::fs;

use frame::{conf_to_iter, exe_generate, ExampleThree};
use frame::{hyper_manager::server::*, route_manager::route::*, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut route = Route::new();
    route.insert("/exam".to_string());
    let mut conf = Config::with_route(route);
    println!("{:#?}", conf.route.search("/exam".to_string()).1);
    let addr_vec = conf.route.addr_vec();
    println!("{:#?}", addr_vec);
    conf.insert_exe(Box::new(ExampleThree));
    let (method_iter, exe_iter) = conf_to_iter!(conf);
    for exe in exe_iter {
        if addr_vec.contains(&exe.0) {
            exe.1.call();
        }
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr, conf).await
}
