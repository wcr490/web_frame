use super::*;
use crate::{midware_generator, midware_method_generator};
use mini_redis::{client as RedisClient, Result as RedisResult};
use tokio::task;

enum FrameType {
    StatelessInfo,
    Auth,
    Unknown,
}

midware_generator!(Redis);
midware_method_generator!(Redis, Priority::P2, |req: RequestType| {
    /* It should have a thread pool int the later version */
    if let RequestType::POST(map) = req.clone() {
        task::block_in_place(|| {
            tokio::runtime::Runtime::new()
                .expect("fail to build a new rt")
                .block_on(redis_connect(map))
                .expect("rt dropped for some errors");
        });
    }

    println!("redis");
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
        FrameType::StatelessInfo => return write_stateless_info(&mut client, map).await,
        FrameType::Auth => {}
    }
    Ok(())
}

async fn type_ident(flag: String) -> Option<FrameType> {
    if flag.is_empty() {
        match flag.as_ref() {
            "Info" => return Some(FrameType::StatelessInfo),
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
        client.set(&k, v.into()).await?;
    }
    Ok(())
}
