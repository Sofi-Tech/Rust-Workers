use bson::Document;
use mongodb::Cursor;

use super::Redis;
use crate::canvas::functions::fetch_buffer;

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
        // uuid:character_cards:buffer or uuid:frames:buffer

        redis.set(&key_name, buffer).unwrap();
        println!("{:?} {url}", key_name);
    }
}
