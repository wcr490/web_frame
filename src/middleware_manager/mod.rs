/*
 * brief Statements before
 * NOTE: Hoping to be more concise comprehensively, we will use these aliases
 *       mw => middleware
 *       MQueue => MiddlewareQueue
 *
 *       we assume that you obey the format of file name
 *       E.g. mw_get.rs
 *            mw_redis.rs
 * */
pub mod mw_queue;

pub mod mw_get;

use super::hyper_manager::request_handler::*;
use super::route_manager::route::*;
use mw_get::*;
use mw_queue::*;

use std::collections::HashMap;
use std::collections::VecDeque;

#[macro_export]
macro_rules! midware_generator {
    () => {};
    ($name: ident) => {
        pub struct $name;
    };
}

#[macro_export]
macro_rules! midware_method_generator {
    () => {};
    ($midware: ident, $priority: expr, $exe: expr) => {
        impl Middleware for $midware {
            fn exe(&self, req: RequestType) -> RequestType {
                $exe(req)
            }
            fn priority(&self) -> Priority {
                $priority
            }
        }
    };
}
#[test]
fn queue() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), "199".to_string());
    let mut q = MwQueue::new();
    q.enqueue(Box::new(Get));
    let res = q.boot(RequestType::GET(map));
    assert_eq!(true, res);
}
