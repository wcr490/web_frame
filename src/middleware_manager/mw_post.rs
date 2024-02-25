use super::*;
use crate::{midware_generator, midware_method_generator};

midware_generator!(Post);
midware_method_generator!(Post, Priority::P1, |req: RequestType| {
    println!("{:#?}", req);
    req
});
