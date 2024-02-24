pub mod example_view;
pub mod simple_view;

use super::hyper_manager::server::full;
use super::route_manager::route::*;
use hyper::{Method, Response};
use std::collections::HashMap;
use std::fs;
