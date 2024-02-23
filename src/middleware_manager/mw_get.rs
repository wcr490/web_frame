/*
 * NOTE: Because of the ambigious definition of what a middleware exactly is, I made that a
 * seperate module and be a part of the middleware queue, noting the highest priority to grab data
 * for advanced middleware
 *
 * */
use super::*;
use crate::{midware_generator, midware_method_generator};

midware_generator!(Get);
midware_method_generator!(Get, Priority::P1, |req| {
    println!("get");
    req
});
