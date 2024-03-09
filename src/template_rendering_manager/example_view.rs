use super::*;
use crate::exe_generator;

exe_generator!(Kk, "/exam".to_string(), Method::POST, {
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./layui/test.html").unwrap(),
    )))
});

exe_generator!(Gg, "/exam/press".to_string(), Method::POST, {
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/press.html").unwrap(),
    )))
});

exe_generator!(Pp, "/exam/redis".to_string(), Method::POST, {
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/redis.html").unwrap(),
    )))
});

exe_generator!(Ss, "/exam/sql".to_string(), Method::POST, {
    Ok::<_, hyper::Error>(Response::new(full(
        fs::read_to_string("./html/sql.html").unwrap(),
    )))
});
