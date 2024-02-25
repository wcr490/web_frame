pub mod request_handler;
pub mod server;

use super::middleware_manager::{mw_get::*, mw_queue::*};
use super::*;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::{Body, Bytes as HttpBytes, Frame};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Uri;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use request_handler::*;
use std::collections::HashMap;
use std::net::SocketAddr;

use futures::lock::Mutex;
use std::sync::Arc;
