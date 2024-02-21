pub mod route;

use super::hyper_manager::server::full;
use super::*;
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::Response;
use std::collections::HashMap;
use std::fs;
