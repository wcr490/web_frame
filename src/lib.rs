pub mod hyper_manager;
pub mod route_manager;

use hyper_manager::server::*;
use route_manager::route::*;

pub struct Config {
    route: Route,
}

impl Config {
    pub fn new() -> Self {
        Config {
            route: Route::new(),
        }
    }
}

macro_rules! route_exec {
    ($method: expr, ($config: expr),*) => {{
        ($method, $config) => Ok($config)
    }};
}
