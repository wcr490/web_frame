use super::*;

#[derive(Clone)]
pub enum RequestType {
    Empty,
    GET(HashMap<String, String>),
    POST,
}
pub async fn req_init(req: &Request<hyper::body::Incoming>) -> RequestType {
    //it's a path
    if req.uri().query() == None {
        return RequestType::Empty;
    }
    //We normally assume that only GET requests have query
    //So
    //If type of the request is GET
    if req.method().eq(&Method::GET) {
        let query = req.uri().query().unwrap();
        let val_vec = query_split(query).await;
        return RequestType::GET(symbol_map(val_vec).await);
    }
    //If type of the request is POST
    else {
        return RequestType::POST;
    }
}
pub async fn query_split(query: &str) -> Vec<&str> {
    let mut vec: Vec<&str> = Vec::new();
    let mut query_cloned = query;
    while let Some(index) = query_cloned.find("&") {
        vec.push(&query_cloned[0..index]);
        query_cloned = &query_cloned[index + 1..];
    }
    vec.push(query_cloned);
    vec
}
pub async fn symbol_map(vec: Vec<&str>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for ele in vec {
        if let Some(symbol) = ele.find("=") {
            map.insert(ele[0..symbol].to_string(), ele[symbol + 1..].to_string());
        }
    }
    map
}
