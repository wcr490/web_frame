use std::borrow::BorrowMut;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use frame::conf_to_iter;
use frame::{hyper_manager::server::*, route_manager::route::*, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut route = Route::new();
    route.insert("/ok".to_string());
    let vec = route.addr_vec();
    let mut conf = Config::with_route(route);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr, conf).await
}
