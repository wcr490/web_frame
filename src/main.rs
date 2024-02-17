use std::borrow::BorrowMut;

use frame::conf_to_iter;
use frame::{hyper_manager::server::*, route_manager::route::*, Config};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut route = Route::new();
    route.insert("112/22/888/aaa".to_string());
    route.insert("112/22/8".to_string());
    route.insert("112/1".to_string());
    let vec = route.addr_vec();
    let mut conf = Config::with_route(route);
    conf.exec
        .insert("112/22/888/aaa".to_string(), Box::new(Test));
    conf.exec.insert("112/22/8".to_string(), Box::new(Test));
    let (mut prefix, mut method) = conf_to_iter!(conf);
    // for (k, v) in prefix {
    //     v.call();
    //     println!("{k}");
    // }
    // println!("{:#?}", vec);
    Ok(())
}

struct Test;
impl Callback for Test {
    fn call(&self) {
        println!("exec");
    }
}
