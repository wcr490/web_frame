pub mod hyper_manager;
pub mod route_manager;

use hyper::Response;
use hyper::{header::Iter, Method};
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

use hyper_manager::server::*;
use route_manager::route::*;

pub struct Config {
    pub route: Route,
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
    pub fn exe(&self) -> &HashMap<String, Exe> {
        &self.exec
    }

    pub fn insert_exe(&mut self, exe: Exe) {
        self.exec.insert(exe.path(), exe);
    }
}

#[macro_export]
macro_rules! conf_to_iter {
    () => {};
    ($config: expr) => {
        (
            $config.method().into_iter(),
            $config.route.exe_map().into_iter(),
        )
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
