#![allow(dead_code)]
#![allow(unused_imports)]
mod caching;
mod canvas;
mod mongo;
mod threads;

use std::{
    error::Error,
    fs::File,
    io::Write,
    process, thread,
    time::{Duration, Instant},
};

use bson::{doc, Document};
use caching::{
    functions::{deserialize_buffer, start_db_caching},
    Redis,
};
use canvas::{
    functions::{draw_card, Card},
    Canvas,
};
use chrono::prelude::*;
use mongo::{
    functions::{get_all_character_cards, get_random_cards, get_three_cards},
    Mongo,
};
use mongodb::Collection;
use threads::asyncpool::create_new_pool;
use tokio::{join, runtime, time::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: get tuple of documents from random_cards function and pass it to
    // generate_drop
    let mongo = mongo::Mongo::new().await.unwrap();
    let client = mongo.get_client();

    // redis caching below
    // **********************************************************************
    // * let redis = Redis::new().unwrap();                                 *
    // * redis.drop_all_keys().unwrap();                                    *
    // * let cursor = get_all_character_cards(&collection).await;           *
    // * start_db_caching(cursor, "character_cards").await;                 *
    // **********************************************************************

    let async_th_pool = create_new_pool(12, "Drop-Generator-Thread-".to_string());
    // todo - get random cards from db
    // todo - don't create new redis on each loop

    for _i in 1..101 {
        let start = Instant::now();
        let redis_connection = caching::Redis::new().unwrap();
        let collection: Collection<Document> =
            client.database("Sofi").collection("character_cards");
        async_th_pool.spawn_ok(async move {
            let _three_cards = get_random_cards(collection).await;
            let _one_uid = _three_cards.0.get("unique_id").unwrap().as_str().unwrap();
            let _two_uid = _three_cards.1.get("unique_id").unwrap().as_str().unwrap();
            let _three_uid = _three_cards.2.get("unique_id").unwrap().as_str().unwrap();
            // println!("{}\n{}\n{}\n_____", _one_uid, _two_uid, _three_uid);
            // todo use uid's to get card from redis
            let images = redis_connection
                .mget(vec![
                    "39e431f3-442c-4bbd-8b7e-e3ea92166c03:character_cards:buffer".to_string(),
                    "59d1fcbf-0f23-4e81-81f8-ce3eda707584:character_cards:buffer".to_string(),
                    "cb78dcb9-108f-4daa-be2a-82c722607981:character_cards:buffer".to_string(),
                ])
                .unwrap();

            let image_one = deserialize_buffer(images[0].clone()).buffer;
            let image_two = deserialize_buffer(images[1].clone()).buffer;
            let image_three = deserialize_buffer(images[2].clone()).buffer;
            let canvas = Canvas::new(1_008, 524);

            // Here we are passing canvas to the draw_card fn so it's ownership will be
            // lost. We can't use it in the next line. So instead we return it from the
            // function and pass it again in 2nd function. This way we don't need to clone
            // or add any lifetime and we can use the canvas in the next line.
            // Not sure if adding lifetime will have any issue or something so before I do
            // research on it, will do it this way.
            let canvas = draw_card(
                canvas,
                Card {
                    image: image_one,
                    frame_url: "./frames/cyan-drop.png".to_string(),
                    gen: 1,
                    name: "Rose".to_string(),
                    series: "Blackpink".to_string(),
                },
                1,
            );
            let canvas = draw_card(
                canvas,
                Card {
                    image: image_two,
                    frame_url: "./frames/purple-drop.png".to_string(),
                    gen: 1,
                    name: "Gojo Satoru".to_string(),
                    series: "Jujutsu Kaisen".to_string(),
                },
                347,
            );
            let mut canvas = draw_card(
                canvas,
                Card {
                    image: image_three,
                    frame_url: "./frames/yellow-drop.png".to_string(),
                    gen: 1,
                    name: "Demon Slayer".to_string(),
                    series: "Nezuko Kamado".to_string(),
                },
                692,
            );

            let drop_image = canvas.data();

            let name = format!("./out/{_i}.{}.png", Utc::now().timestamp_millis());
            let mut file = File::create(name).unwrap();
            let bytes = drop_image.as_bytes();
            file.write_all(bytes).unwrap();
            println!("Time taken: {:?}", Instant::now() - start);
        });
    }
    sleep(Duration::from_millis(1000 * 60 * 5)).await;
    Ok(())
}
