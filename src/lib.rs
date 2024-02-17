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
        let exec: HashMap<String, Exe> = HashMap::new();

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
/*
 * macro_rules! route_handlers {
    ($method:expr, $handlers:expr) => {
        match ($method, req.uri().path()) {
            $(
                (method, path) if method == &$method && $handlers.contains_key(path) => {
                    let handler = $handlers.get(path).unwrap();
                    handler()
                },
            )*
            _ => {
                let mut not_found = Response::new(empty());
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    };
}
*/

#[macro_export]
macro_rules! conf_to_iter {
    () => {};
    ($config: expr) => {
        ($config.method().into_iter(), $config.exec.iter())
    };
}
