use bson::Document;
use mongodb::Cursor;
use serde::{Deserialize, Serialize};

use super::Redis;
use crate::canvas::functions::fetch_buffer;

#[derive(Serialize, Deserialize, Debug)]
pub struct BufferInstance {
    pub buffer_type: String,
    pub buffer: Vec<u8>,
}

pub struct CardStruct {
    pub id: String,
    pub name: String,
    pub image: String,
    pub released: bool,
}

pub async fn start_db_caching(redis: &Redis, mut cards: Cursor<Document>, wild_key: &str) {
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
        let buffer = fetch_buffer(url).await;
        println!("Caching: {}", key_name);
        redis
            .set(
                &key_name,
                serialize_buffer(BufferInstance {
                    buffer_type: wild_key.to_string(),
                    buffer,
                }),
            )
            .unwrap();
    }
}

pub fn serialize_buffer(value: BufferInstance) -> String {
    serde_json::to_string(&value).unwrap()
}

pub fn deserialize_buffer(value: String) -> BufferInstance {
    serde_json::from_str(&value).unwrap()
}
