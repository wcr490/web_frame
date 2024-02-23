pub mod hyper_manager;
pub mod middleware_manager;
pub mod route_manager;
pub mod template_rendering_manager;

use hyper::Method;
use hyper_manager::server::*;
use route_manager::route::*;
use std::collections::HashMap;
use template_rendering_manager::simple_view::*;

/// essential struct
/// used to correspond and relate different threads
pub struct Config {
    exec: HashMap<String, Cb>,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        let mut config = Config::new();
        let exe_iter = self.exe().into_iter();
        for (k, v) in exe_iter {
            config.exec.insert(k.to_string(), Cb(v.0.clone()));
        }
        config
    }
}
impl Config {
    /// allow to generate an empty Config(normally not)
    pub fn new() -> Self {
        Config {
            exec: HashMap::new(),
        }
    }
    /// more recommended way to create a Config with a prepared Route
    pub fn with_route(route: Route) -> Self {
        let exec = route.exe_map();
        let mut exe_map: HashMap<_, Cb> = HashMap::new();
        for (k, v) in exec {
            exe_map.insert(k, Cb(v.clone()));
        }
        Config { exec: exe_map }
    }

    pub fn exe(&self) -> &HashMap<String, Cb> {
        &self.exec
    }
}
