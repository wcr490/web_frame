use std::borrow::BorrowMut;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use hyper::Response;
use std::fs;

use frame::{conf_to_iter, exe_generate, ExampleThree};
use frame::{hyper_manager::server::*, route_manager::route::*, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}
