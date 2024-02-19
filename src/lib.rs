pub mod hyper_manager;
pub mod route_manager;

use hyper::Response;
use hyper::{header::Iter, Method};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::sync::Arc;

use hyper_manager::server::*;
use route_manager::route::*;

pub struct Config {
    pub exec: HashMap<String, Cb>,
    method: HashMap<String, Method>,
}
impl Clone for Config {
    fn clone(&self) -> Self {
        let mut config = Config::new();
        let method_iter = self.method().into_iter();
        let exe_iter = self.exe().into_iter();
        for (k, v) in method_iter {
            config.method.insert(k.to_string(), v.clone());
        }
        for (k, v) in exe_iter {
            config
                .exec
                .insert(k.to_string(), Cb(Box::new(DefaultCallback)));
        }
        config
    }
}
pub struct Cb(Box<dyn Callback>);
unsafe impl Sync for Cb {}
unsafe impl Send for Cb {}
pub struct ArcConfig(Arc<Config>);
unsafe impl Send for Config {}
unsafe impl Sync for Config {}
unsafe impl Send for ArcConfig {}
unsafe impl Sync for ArcConfig {}
impl Config {
    pub fn new() -> Self {
        Config {
            exec: HashMap::new(),
            method: HashMap::new(),
        }
    }
    pub fn with_route(route: Route) -> Self {
        let exec = route.exe_map();
        Config {
            exec: HashMap::new(),
            method: HashMap::new(),
        }
    }
    pub fn method(&self) -> HashMap<String, Method> {
        self.method.clone()
    }
    pub fn exe(&self) -> &HashMap<String, Cb> {
        &self.exec
    }
}

#[macro_export]
macro_rules! conf_to_iter {
    () => {};
    ($config: expr) => {
        ($config.method().into_iter(), $config.exec.into_iter())
    };
}
#[macro_export]
macro_rules! exe_generate {
    () => {};
    ($name: ident, $path: expr, $body: block) => {
        pub struct $name;
        impl Callback for $name {
            fn call(&self) -> Result<Resp, hyper::Error> {
                $body
            }
            fn path(&self) -> String {
                $path
            }
        }
    };
    ($conf: expr,$name: ident, $path: expr, $body: block) => {
        pub struct $name;
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

exe_generate!(ExampleTwo, "ex".to_string(), {
    println!("example_two");
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("hello.html").unwrap(),
    )))
});
exe_generate!(conf, ExampleThree, "/exam".to_string(), {
    println!("three");
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("hello.html").unwrap(),
    )))
});
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn b() {}
}
