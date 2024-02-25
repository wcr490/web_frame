use super::*;
use crate::{midware_generator, midware_method_generator};

midware_generator!(POST);
midware_method_generator!(POST, Priority::P1, |req| {
    println!("post");
    req
});
