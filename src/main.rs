use frame::route_manager::route::*;
use std::{
    ops::Deref,
    time::{Duration, Instant},
};
#[tokio::main]
async fn main() {
    let mut route: Route = Route::new();
    route.insert("aad/ss/kk".to_string());
    let (exist, vec) = route.search("aad/ss/kk/tt".to_string());
    println!("{exist}");
    println!("the same part is {:#?}", vec);
}
