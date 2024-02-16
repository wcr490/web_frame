use frame::{hyper_manager::server::*, route_manager::route::*, Config};
use http_body_util::combinators::BoxBody;
use std::net::SocketAddr;
use std::{
    ops::Deref,
    time::{Duration, Instant},
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = Instant::now();
    let mut route: Route = Route::new();
    route.insert("aad/ss/kk".to_string());
    let (exist, vec) = route.search("aad/ss/kk/tt".to_string());
    // println!("{exist}");
    // println!("the same part is {:#?}", vec);
    let dur = now.elapsed().as_micros();
    // println!("it uses {:?} micros", dur);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let config = Config::new();

    let ret = build(addr, config).await;
    ret
}
