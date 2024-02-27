use super::*;
use crate::{midware_generator, midware_method_generator};
use mini_redis::{client as RedisClient, Result as RedisResult};
use tokio::task;

enum FrameType {
    StatelessInfoWrite,
    StatelessInfoRead,
    Auth,
    Unknown,
}

midware_generator!(Redis);
midware_method_generator!(Redis, Priority::P2, |req: RequestType| {
    /* It should have a thread pool int the later version */
    if let RequestType::POST(map) = req.clone() {
        task::block_in_place(|| {
            tokio::runtime::Runtime::new()
                .expect("redis: fail to build a new rt")
                .block_on(redis_connect(map))
                .expect("redis: rt dropped for some errors");
        });
    }
    req
});

async fn redis_connect(map: HashMap<String, String>) -> RedisResult<()> {
    let mut client = RedisClient::connect("127.0.0.1:6379").await?;
    let frame_type: FrameType = match map.get("redis_flag") {
        None => return Ok(()),
        Some(flag) => {
            if let Some(t) = type_ident(flag.to_string()).await {
                t
            } else {
                return Ok(());
            }
        }
    };
    match frame_type {
        FrameType::Unknown => {}
        FrameType::StatelessInfoWrite => return write_stateless_info(&mut client, map).await,
        FrameType::StatelessInfoRead => return read_stateless_info(&mut client, map).await,
        FrameType::Auth => {}
    }
    Ok(())
}

async fn type_ident(flag: String) -> Option<FrameType> {
    if !flag.is_empty() {
        match flag.as_ref() {
            "InfoWrite" => return Some(FrameType::StatelessInfoWrite),
            "InfoRead" => return Some(FrameType::StatelessInfoRead),
            "Auth" => return Some(FrameType::Auth),
            _ => return Some(FrameType::Unknown),
        }
    } else {
        return None;
    }
}

async fn write_stateless_info(
    client: &mut RedisClient::Client,
    map: HashMap<String, String>,
) -> RedisResult<()> {
    for (k, v) in map {
        if k == "flag" || k == "redis_flag" {
            continue;
        }
        println!("redis: write {} to {}", v, k);
        client.set(&k, v.into()).await?;
    }
    Ok(())
}
async fn read_stateless_info(
    client: &mut RedisClient::Client,
    map: HashMap<String, String>,
) -> RedisResult<()> {
    if let Some(v) = map.get("redis_read") {
        let mut ret: HashMap<String, String> = HashMap::new();
        let vec = split_comma(v.to_string()).await;
        println!("{:#?}", vec);
        for k in vec {
            if let Some(v) = client.get(k.as_ref()).await? {
                ret.insert(
                    k.to_string(),
                    String::from_utf8(v.to_vec()).expect("fail to parse"),
                );
            }
        }
        println!("{:#?}", ret);
    }
    Ok(())
}
async fn split_comma(str: String) -> Vec<String> {
    let mut tmp = str.clone();
    let mut vec: Vec<String> = Vec::new();
    while let Some(comma) = tmp.find("%2C") {
        vec.push(tmp[0..comma].to_string());
        tmp = tmp[comma + 3..].to_string();
    }
    vec.push(tmp);
    vec
}
