use super::*;

#[derive(Clone, Debug)]
pub enum RequestType {
    GetEmpty,
    PostEmpty,
    GET(HashMap<String, String>),
    POST(HashMap<String, String>),
}

pub async fn req_init(req: Request<hyper::body::Incoming>) -> RequestType {
    //We normally assume that only GET requests have query
    //So
    //If type of the request is GET
    if req.method().eq(&Method::GET) {
        if let Some(query) = req.uri().query() {
            let val_vec = query_split(query).await;
            return RequestType::GET(symbol_map(val_vec).await);
        } else {
            return RequestType::GetEmpty;
        }
    }
    //If type of the request is POST
    else {
        if let Ok(bytes_collected) = req.collect().await {
            let query =
                String::from_utf8(bytes_collected.to_bytes().to_vec()).expect("Fail to parse");
            let val_vec = query_split(&query).await;
            return RequestType::POST(symbol_map(val_vec).await);
        } else {
            RequestType::PostEmpty
        }
    }
}
pub async fn query_split(query: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let mut query_cloned = query;
    while let Some(index) = query_cloned.find("&") {
        vec.push(query_cloned[0..index].to_string());
        query_cloned = &query_cloned[index + 1..];
    }
    vec.push(query_cloned.to_string());
    vec
}
pub async fn symbol_map(vec: Vec<String>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for ele in vec {
        if let Some(symbol) = ele.find("=") {
            map.insert(ele[0..symbol].to_string(), ele[symbol + 1..].to_string());
        }
    }
    map
}
