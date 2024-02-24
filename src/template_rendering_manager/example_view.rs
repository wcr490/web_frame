use super::*;
use crate::exe_generator;

exe_generator!(Kk, "/exam".to_string(), Method::POST, {
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/hello.html").unwrap(),
    )))
});

exe_generator!(Gg, "/exam/gg".to_string(), Method::POST, {
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/hello.html").unwrap(),
    )))
});
