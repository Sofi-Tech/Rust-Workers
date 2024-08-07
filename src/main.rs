#![allow(dead_code)]
#![allow(unused_imports)]
mod caching;
mod canvas;
use rand::prelude::IteratorRandom;
mod mongo;

mod tcp;
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
use rand::seq::SliceRandom;
use tcp::{message_handler::Payload, Server};
use threads::asyncpool::create_new_pool;
use tokio::{join, runtime, time::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let server = Server::new("Sofi".to_string());
    // server.bind("127.0.0.1:3000").await?;

    // sleep(Duration::from_millis(1000 * 5)).await;

    // server
    //     .send(
    //         "a".to_string(),
    //         Payload {
    //             payload: "Hello from rust!".to_string(),
    //         },
    //         Some(true),
    //     )
    //     .await?;

    // TODO: get tuple of documents from random_cards function and pass it to
    // generate-drop
    let mongo = mongo::Mongo::new().await.unwrap();
    let client = mongo.get_client();
    // let collection = client.database("Sofi").collection("character_cards");
    // // redis caching below
    // // **********************************************************************
    // let redis = Redis::new().unwrap();
    // redis.drop_all_keys().unwrap();
    // let cursor = get_all_character_cards(&collection).await;
    // start_db_caching(cursor, "character_cards").await;
    // **********************************************************************

    let async_th_pool = create_new_pool(12, "Drop-Generator-Thread-".to_string());
    // // todo - get random cards from db
    // // todo - don't create new redis on each loop

    let redis_connection = caching::Redis::new().unwrap();
    let connection = redis_connection.copy();

    let collection: Collection<Document> = client.database("Sofi").collection("character_cards");
    async_th_pool.spawn_ok(async move {
        let start = Instant::now();
        let _three_cards = get_random_cards(collection).await;
        let _one_uid = _three_cards.0.get("unique_id").unwrap().as_str().unwrap();
        let _two_uid = _three_cards.1.get("unique_id").unwrap().as_str().unwrap();
        let _three_uid = _three_cards.2.get("unique_id").unwrap().as_str().unwrap();

        // let one = format!("{_one_uid}:character_cards:buffer");
        let one = "8a89afd4-36c2-476b-929c-d050e83b0578:character_cards:buffer".to_string();
        // let two = format!("{_two_uid}:character_cards:buffer");
        let two = "3286f1af-e030-474b-9d34-cb4249196952:character_cards:buffer".to_string();
        // let three = format!("{_three_uid}:character_cards:buffer");
        let three = "3343cf53-bacd-49d0-8240-6dea019be172:character_cards:buffer".to_string();
        let images = connection.mget(vec![one, two, three]).unwrap();

        let image_one = deserialize_buffer(&images[0]).buffer;
        let image_two = deserialize_buffer(&images[1]).buffer;
        let image_three = deserialize_buffer(&images[2]).buffer;
        let canvas = Canvas::new(1_008, 524);

        let elements = vec![
            "blue", "brown", "cyan", "green", "grey", "purple", "red", "yellow",
        ];

        let random_elements = elements
            .choose_multiple(&mut rand::thread_rng(), 3)
            .collect::<Vec<_>>();

        let random_numbers = (1..1024).choose_multiple(&mut rand::thread_rng(), 3);

        let canvas = draw_card(
            canvas,
            Card {
                image: image_one,
                element: random_elements[0].to_string(),
                gen: random_numbers[0],
                name: _three_cards.0.get("name").unwrap().as_str().unwrap(),
                series: _three_cards.0.get("series_name").unwrap().as_str().unwrap(),
            },
            1.,
            1.,
            297.0,
            465.0,
        );
        let canvas = draw_card(
            canvas,
            Card {
                image: image_two,
                element: random_elements[1].to_string(),
                gen: random_numbers[1],
                name: _three_cards.1.get("name").unwrap().as_str().unwrap(),
                series: _three_cards.1.get("series_name").unwrap().as_str().unwrap(),
            },
            347.,
            1.,
            297.0,
            465.0,
        );
        let mut canvas = draw_card(
            canvas,
            Card {
                image: image_three,
                element: random_elements[2].to_string(),
                gen: random_numbers[2],
                name: _three_cards.2.get("name").unwrap().as_str().unwrap(),
                series: _three_cards.2.get("series_name").unwrap().as_str().unwrap(),
            },
            693.,
            1.,
            297.0,
            465.0,
        );

        let drop = canvas.webp();

        let name = format!("./out/{}.{}.webp", 1, Utc::now().timestamp_millis());

        let mut file = File::create(name).unwrap();

        file.write_all(&drop).unwrap();
        println!("Time taken: {:?}", Instant::now() - start);
    });

    sleep(Duration::from_millis(1000 * 60 * 5)).await;
    Ok(())
}
