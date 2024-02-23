pub mod hyper_manager;
pub mod middleware_manager;
pub mod route_manager;

use hyper::Method;
use hyper_manager::server::*;
use route_manager::route::*;
use std::collections::HashMap;

// alias to keep file tidy
/// Specially used by Config to implement Clone
pub struct Cb(pub Box<dyn Callback>);
///neccessary trait due to multithreading
unsafe impl Sync for Cb {}
unsafe impl Send for Cb {}

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

/// A macro used to automatically register a struct which has been implemented Callback
///
/// # Parameter
/// * $name - name of the struct
/// * $path - path in the route
/// * $body - Exe content
///
/// # Return
/// * struct $name
#[macro_export]
macro_rules! exe_generator {
    () => {};

    ($name: ident, $path: expr, $method: expr, $body: block) => {
        #[derive(Clone)]
        pub struct $name;
        impl Callback for $name {
            fn call(&self) -> Result<Resp, hyper::Error> {
                $body
            }
            fn method(&self) -> Method {
                $method
            }
            fn path(&self) -> String {
                $path
            }
            fn box_clone(&self) -> Exe {
                Box::new((*self).clone())
            }
        }
    };
}
