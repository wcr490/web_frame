use futures::lock::Mutex;
use std::borrow::BorrowMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use hyper::Response;
use std::fs;

use frame::{conf_to_iter, exe_generate, ExampleThree};
use frame::{hyper_manager::server::*, route_manager::route::*, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut route = Route::new();
    route.insert_path("/exam".to_string());
    exe_generate!(Exam, "/exam".to_string(), {
        println!("success");
        Ok::<_, hyper::Error>(Response::new(full(
            fs::read_to_string("hello.html").unwrap(),
        )))
    });
    route.insert_exe(Box::new(Exam), "/exam".to_string());
    // println!("{:#?}", route.search("/example".to_string()));
    // println!("{:#?}", route.addr_vec());

    let mut conf = Config::with_route(route);

    run_server(addr, conf).await
}
