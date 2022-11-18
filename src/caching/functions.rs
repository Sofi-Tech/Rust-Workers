use std::{
    error::Error,
    process,
    time::{Duration, Instant},
};

use bson::Document;
use chrono::prelude::*;
use futures::{stream, StreamExt};
use mongodb::Cursor;
use redis::RedisError;
use serde::{Deserialize, Serialize};
use tokio::runtime;

use super::Redis;
use crate::{
    caching,
    canvas::request::{fetch_buffer, Request},
    threads::asyncpool,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferInstance {
    pub buffer_type: String,
    pub buffer: Vec<u8>,
}

pub struct CardStruct {
    pub uid: String,
    pub url: String,
    pub key_name: String,
}

pub async fn start_db_caching(mut cards: Cursor<Document>, wild_key: &str) {
    let rt = runtime::Builder::new_multi_thread()
        .thread_name("Caching-Thread")
        .worker_threads(100)
        .enable_all()
        .build()
        .unwrap();

    let mut card_vec: Vec<CardStruct> = Vec::new();
    while cards.advance().await.unwrap() {
        let url = cards
            .current()
            .get("url")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap();
        let uid = cards
            .current()
            .get("unique_id")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap();
        let key_name = format!("{}:{}:buffer", uid, wild_key);
        let card = CardStruct {
            uid: uid.to_string(),
            url: url.to_string(),
            key_name: key_name.to_string(),
        };
        card_vec.push(card);
        println!("size: {}", card_vec.len());
    }
    with_runtime(&rt, card_vec).await;
}

pub fn serialize_buffer(value: BufferInstance) -> String {
    serde_json::to_string(&value).unwrap()
}

pub fn deserialize_buffer(value: String) -> BufferInstance {
    serde_json::from_str(&value).unwrap()
}

async fn concurrent(card_vec: Vec<CardStruct>) {
    let fetches = futures::stream::iter(card_vec.into_iter().map(|card| async move {
        let redis_connection = caching::Redis::new().unwrap();
        let time = Instant::now();
        match reqwest::get(&card.url).await {
            Ok(resp) => match resp.bytes().await {
                Ok(buffer) => {
                    let key_name = card.key_name.clone();
                    let buffer_instance = BufferInstance {
                        buffer_type: "image".to_string(),
                        buffer: buffer.to_vec(),
                    };
                    let serialized_buffer = serde_json::to_string(&buffer_instance).unwrap();
                    println!(
                        "Time taken: {:?} {}",
                        Instant::now() - time,
                        Utc::now().timestamp_millis()
                    );
                    redis_connection.set(key_name, serialized_buffer).unwrap();
                }
                Err(_) => println!("ERROR reading {}", card.url),
            },
            Err(_) => println!("ERROR downloading {}", card.url),
        }
    }))
    .buffer_unordered(12)
    .collect::<Vec<()>>();
    println!("Waiting...");
    fetches.await;
}

async fn with_runtime(rt: &runtime::Runtime, card_vec: Vec<CardStruct>) {
    let redis_connection = caching::Redis::new().unwrap();
    for (i, card) in card_vec.iter().enumerate() {
        let redis_clone = redis_connection.copy();
        let key_name = card.key_name.clone();
        let url = card.url.clone();
        let handle = rt.spawn(async move {
            let time = Instant::now();
            let buffer = fetch_buffer(&url).await;
            let buffer_instance = BufferInstance {
                buffer_type: "image".to_string(),
                buffer: buffer.to_vec(),
            };
            let serialized_buffer = serde_json::to_string(&buffer_instance).unwrap();
            println!(
                "{i} Time taken: {:?} {}",
                Instant::now() - time,
                Utc::now().timestamp_millis()
            );
            redis_clone.set(key_name, serialized_buffer).unwrap();
        });
        let _ = tokio::time::timeout(Duration::from_millis(0), handle).await;
        // if i == 1 {
        //     // sleep(Duration::from_millis(1000 * 60)).await;
        //     break;
        // }
    }
}
