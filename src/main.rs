#![allow(dead_code)]
#![allow(unused_imports)]
mod caching;
mod canvas;
mod mongo;

use std::{
    error::Error,
    fs::File,
    io::Write,
    thread,
    time::{Duration, Instant},
};

use bson::Document;
use caching::functions::deserialize_buffer;
use canvas::{
    functions::{draw_card, fetch_buffer, Card},
    Canvas,
};
use chrono::prelude::*;
use mongo::Mongo;
use tokio::{join, runtime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let redis_connection = caching::Redis::new()?;
    // let mongo = mongo::Mongo::new().await?;
    // redis_connection.drop_all_keys()?;
    // let cards =
    //     mongo::functions::get_all_character_cards(&mongo.get_collection("
    // character_cards")).await; caching::functions::start_db_caching(&
    // redis_connection, cards, "character_cards").await;

    // let val = redis_connection
    //     .get("a3d6ded1-c919-46bb-93df-379a9f12174d:character_cards:buffer")
    //     .unwrap();
    // caching::functions::deserialize_buffer(val);

    let start = std::time::Instant::now();
    let images = redis_connection
        .mget(vec![
            "39e431f3-442c-4bbd-8b7e-e3ea92166c03:character_cards:buffer".to_string(),
            "59d1fcbf-0f23-4e81-81f8-ce3eda707584:character_cards:buffer".to_string(),
            "cb78dcb9-108f-4daa-be2a-82c722607981:character_cards:buffer".to_string(),
        ])
        .unwrap();

    //  println!("{:?}", images);
    let image_one = deserialize_buffer(images[0].clone()).buffer;
    let image_two = deserialize_buffer(images[1].clone()).buffer;
    let image_three = deserialize_buffer(images[2].clone()).buffer;

    let canvas = Canvas::new(1_008, 524);
    // Here we are passing canvas to the draw_card fn so it's ownership will be
    // lost. We can't use it in the next line. So instead we return it from the
    // function and pass it again in 2nd function. This way we don't need to
    // clone or add any lifetime and we can use the canvas in the next line.
    // Not sure if adding lifetime will have any issue or something so before I
    // do research on it, will do it this way.
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

    let name = format!("./out/{}.png", Utc::now().timestamp_millis());
    let mut file = File::create(name).unwrap();
    let bytes = drop_image.as_bytes();
    file.write_all(bytes).unwrap();
    println!("Time taken: {:?}", start.elapsed());
    Ok(())
}
