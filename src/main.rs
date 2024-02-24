use futures::lock::Mutex;
use hyper::Method;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use hyper::Response;
use std::fs;

use frame::template_rendering_manager::example_view::*;
use frame::{
    exe_generator,
    hyper_manager::request_handler::*,
    hyper_manager::server::*,
    middleware_manager::{mw_get::*, mw_queue::*},
    mw_queue_generator, mw_queue_map_generator,
    route_manager::route::*,
    Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /* Middleware Queue Testing */
    let mut q = MwQueue::new();
    mw_queue_generator!(q, Get);
    let mut q_map = HashMap::new();
    mw_queue_map_generator!(q_map, Flag("just_get_it".to_string()) => q);

    /* Server Testing */
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut route = Route::new();
    route.insert_path("/exam".to_string());
    route.insert_path("/exam/gg".to_string());
    route.insert_exe(Box::new(Kk), "/exam".to_string());
    route.insert_exe(Box::new(Gg), "/exam/gg".to_string());
    let conf = Config::with_route_queue(route, q_map);

    run_server(addr, conf).await
}
