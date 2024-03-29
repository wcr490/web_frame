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

pub mod mw_get;
pub mod mw_post;
pub mod mw_queue;
pub mod mw_redis;
// pub mod mw_sql;

use super::hyper_manager::request_handler::*;
use super::route_manager::route::*;
use mw_get::*;
use mw_queue::*;
use once_cell::sync::OnceCell;
use serde_json::json;
use serde_json::Value;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::Pool;
use sqlx::{Column, Row};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

#[macro_export]
macro_rules! midware_generator {
    () => {};
    ($name: ident) => {
        #[derive(Clone)]
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
            fn box_clone(&self) -> Box<dyn Middleware> {
                Box::new((*self).clone())
            }
        }
    };
}
