use frame::sql_pool_generator;
use futures::lock::Mutex;
use hyper::Method;
use mini_redis::{client, Result as RedisResult};
use once_cell::sync::OnceCell;

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
    middleware_manager::{mw_get::*, mw_post::*, mw_queue::*, mw_redis::*},
    mw_queue_generator, mw_queue_map_generator,
    route_manager::route::*,
    Config,
};

sql_pool_generator!();
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /* Sql Testing  */
    let pool = set_db_pool("mysql://root:852200gH@localhost:3306/test").await;
    db_pool();
    println!(
        "succeed connecting to DB and creating a thread pool:\n{:#?}",
        DB
    );
    /* Redis Testing */
    // let mut client = client::connect("127.0.0.1:6379").await?;
    // client.set("hello world", "hey".into()).await?;
    // let res = client.get("hello world").await?;
    // println!("{:#?}", res);

    /* Middleware Queue Testing */
    let mut get_q = MwQueue::new();
    mw_queue_generator!(get_q, Get);
    let mut post_q = MwQueue::new();
    mw_queue_generator!(post_q, Post, Redis);
    let mut q_map = HashMap::new();
    mw_queue_map_generator!(q_map, Flag("just_get_it".to_string()) => get_q, Flag("redis_flag".to_string()) => post_q);

    /* Server Testing */
    let addr = SocketAddr::from(([192, 168, 3, 148], 3000));
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut route = Route::new();
    route.insert_path("/exam".to_string());
    route.insert_path("/exam/press".to_string());
    route.insert_path("/exam/post".to_string());
    route.insert_exe(Box::new(Kk), "/exam".to_string());
    route.insert_exe(Box::new(Gg), "/exam/press".to_string());
    route.insert_exe(Box::new(Pp), "/exam/post".to_string());
    let conf = Config::with_route_queue(route, q_map);

    run_server(addr, conf).await
}
