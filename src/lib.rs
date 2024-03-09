pub mod hyper_manager;
pub mod middleware_manager;
pub mod route_manager;
pub mod template_rendering_manager;
use once_cell::sync::OnceCell;

use hyper::Method;
use hyper_manager::server::*;
use middleware_manager::mw_queue::*;
use route_manager::route::*;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::collections::HashMap;
use template_rendering_manager::example_view::*;
use template_rendering_manager::simple_view::*;

// pub static DB: OnceCell<MySqlPool> = OnceCell::new();
pub static mut REDIS_NORMAL_FILE: OnceCell<std::fs::File> = OnceCell::new();
pub static mut REDIS_AUTH_FILE: OnceCell<std::fs::File> = OnceCell::new();

/// essential struct
/// used to correspond and relate different threads
pub struct Config {
    view: ViewMap,
    queue: MwQueueMap,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        let mut config = Config::new();
        let exe_iter = self.exe().into_iter();
        for (k, v) in exe_iter {
            config.view.insert(k.to_string(), ViewCb(v.0.clone()));
        }
        config
    }
}
impl Config {
    /// allow to generate an empty Config(normally not)
    pub fn new() -> Self {
        Config {
            view: HashMap::new(),
            queue: HashMap::new(),
        }
    }
    /// more recommended way to create a Config with a prepared Route
    pub fn with_route(route: Route) -> Self {
        let exec = route.exe_map();
        let mut exe_map: HashMap<_, ViewCb> = HashMap::new();
        for (k, v) in exec {
            exe_map.insert(k, ViewCb(v.clone()));
        }
        Config {
            view: exe_map,
            queue: HashMap::new(),
        }
    }
    pub fn with_route_queue(route: Route, queue: MwQueueMap) -> Self {
        let mut conf = Config::with_route(route);
        conf.queue = queue;
        conf
    }

    pub fn exe(&self) -> &HashMap<String, ViewCb> {
        &self.view
    }
}
