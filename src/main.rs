#![allow(dead_code)]
mod canvas;
use std::{fs::File, io::Write};

use canvas::{
    functions::{draw_card, fetch_buffer, Card},
    Canvas,
};

mod mongo;
mod redis;

use std::{
    error::Error,
    time::{Duration, Instant},
};

use chrono::prelude::*;
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let redis_connection = redis::Redis::new()?;
    // let mongo = mongo::Mongo::new().await?;
    let start1 = Instant::now();
    redis_connection
        .get("59d1fcbf-0f23-4e81-81f8-ce3eda707584:character_cards:buffer")
        .unwrap();
    let duration: Duration = start1.elapsed();
    println!("GET is: {:?}", duration);
    // TODO: get tuple of documents from random_cards function and pass it to
    // generate_drop

    let (image_one, image_two, image_three) = join!(
        fetch_buffer(
            "https://cdn.w1st.xyz/cards/characters/42739898-0dc5-43ec-b918-889fd1a993b0.jpg"
        ),
        fetch_buffer(
            "https://cdn.w1st.xyz/cards/characters/1e364732-dfee-4672-bc0e-75796d3f9f78.jpg"
        ),
        fetch_buffer(
            "https://cdn.w1st.xyz/cards/characters/358445c8-0bd8-43ff-943b-4bdfa1264275.jpg"
        )
    );

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
    )
    .await;
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
    )
    .await;
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
    )
    .await;

    let drop_image = canvas.data();

    let name = format!("./out/{}.png", Utc::now().timestamp_millis());
    let mut file = File::create(name).unwrap();
    let bytes = drop_image.as_bytes();
    file.write_all(bytes).unwrap();

    // let mongo_client: Mongo = Mongo::new().await.unwrap();
    // let rt = runtime::Builder::new_multi_thread()
    //     .thread_name("Sofi Worker Pool")
    //     .worker_threads(12)
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // //thread_job();
    // runtime_thread_job(&mongo_client, &rt).await;
    // tokio_thread_job(&mongo_client).await;
    // sleep(Duration::from_millis(1000 * 60)).await;
    // println!("60s have elapsed");
    Ok(())
}
