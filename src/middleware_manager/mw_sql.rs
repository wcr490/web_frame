use super::*;
use crate::{midware_generator, midware_method_generator};

midware_generator!(Sql);
midware_method_generator!(Sql, Priority::P3, |req| {
    println!("Sql");
    req
});
