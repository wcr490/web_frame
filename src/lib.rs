pub mod hyper_manager;
pub mod route_manager;

use hyper::Method;
use std::collections::HashMap;

use hyper_manager::server::*;
use route_manager::route::*;

pub struct Config {
    route: Route,
    exec: HashMap<String, Exe>,
    method: Method,
}

impl Config {
    pub fn new() -> Self {
        Config {
            route: Route::new(),
            exec: HashMap::new(),
            method: Method::POST,
        }
    }
}

macro_rules! route_exec {
    () => {};
    ($config: ident: Config, $($rest:tt)*) => {
        let mut map = $config.exec;
        let mut iter = map.iter();
        let key;
        let val;
        if let Some(k,v) = iter.next() {
            let key = k;
            let val = v;
        }
        {
            ($config.method, key) => {
                Ok(v)
            }
        }

    }
}
