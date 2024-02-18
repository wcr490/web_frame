pub mod hyper_manager;
pub mod route_manager;

use hyper::{header::Iter, Method};
use std::collections::HashMap;

use hyper_manager::server::*;
use route_manager::route::*;

pub struct Config {
    route: Route,
    pub exec: HashMap<String, Exe>,
    method: HashMap<String, Method>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            route: Route::new(),
            exec: HashMap::new(),
            method: HashMap::new(),
        }
    }
    pub fn with_route(route: Route) -> Self {
        let exec = route.exe_map();

        Config {
            route,
            exec: HashMap::new(),
            method: HashMap::new(),
        }
    }
    pub fn method(&self) -> HashMap<String, Method> {
        self.method.clone()
    }
}
#[macro_export]
macro_rules! conf_to_iter {
    () => {};
    ($config: expr) => {
        ($config.method().into_iter(), $config.exec.iter())
    };
}
#[macro_export]
macro_rules! exe_generate {
    () => {};
    ($name: ident, $path: expr, $body: block) => {
        struct $name;
        impl Callback for $name {
            fn call(&self) -> Result<Resp, hyper::Error> {
                $body
            }
            fn path(&self) -> String {
                $path
            }
        }
    };
}
