use serde::{Deserialize, Serialize};
use std::io::Write;

use super::*;
use crate::{midware_generator, midware_method_generator, REDIS_AUTH_FILE, REDIS_NORMAL_FILE};
use mini_redis::{client as RedisClient, Result as RedisResult};
use tokio::task;

enum FrameType {
    StatelessInfoWrite,
    StatelessInfoRead,
    AuthWrite,
    AuthRead,
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
        FrameType::AuthWrite => return write_auth(&mut client, map).await,
        FrameType::AuthRead => {}
    }
    Ok(())
}

async fn type_ident(flag: String) -> Option<FrameType> {
    if !flag.is_empty() {
        match flag.as_ref() {
            "InfoWrite" => return Some(FrameType::StatelessInfoWrite),
            "InfoRead" => return Some(FrameType::StatelessInfoRead),
            "AuthWrite" => return Some(FrameType::AuthWrite),
            "AuthRead" => return Some(FrameType::AuthRead),
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
        client.set(&k, v.clone().into()).await?;
        unsafe {
            REDIS_NORMAL_FILE
                .get_mut()
                .expect("fail to get redis snapshot file")
                .write_all(format!("{}:{}\n", k, v).as_bytes())?;
            println!("write");
        }
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
#[derive(Serialize, Deserialize)]
pub struct Auth {
    usr: String,
    pwd: String,
    check_stamp: String,
}
async fn write_auth(
    client: &mut RedisClient::Client,
    map: HashMap<String, String>,
) -> RedisResult<()> {
    let auth = map
        .get("authorization")
        .expect("fail to find the infomation of authorization");
    let (usr, pwd) = split_dot(auth).await;
    unsafe {
        REDIS_AUTH_FILE
            .get()
            .expect("fail to access to redis authorization file")
            .write_all(
                serde_json::to_string(&Auth {
                    usr,
                    pwd,
                    check_stamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        .to_string(),
                })
                .expect("fail to convert info into json")
                .as_bytes(),
            )?;
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
async fn split_dot(str: &String) -> (String, String) {
    let (usr, pwd) = str.split_at(str.find(".").expect("authorization has a wrong format"));
    (usr.to_string(), pwd.to_string())
}
