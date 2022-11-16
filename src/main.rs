#![allow(dead_code)]
mod canvas;
use std::{
    fs::File,
    io::{Read, Write},
    thread,
};

use canvas::Canvas;

mod mongo;
use std::{
    error::Error,
    time::{Duration, Instant},
};

use bson::Document;
use canvas::functions::fetch_buffer;
use chrono::prelude::*;
use mongo::Mongo;
use tokio::runtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let buf = fetch_buffer(
        "https://cdn.w1st.xyz/cards/characters/1e364732-dfee-4672-bc0e-75796d3f9f78.jpg",
    )
    .await;

    let mut frame = File::open("./frames/Blue No Space.png").unwrap();
    let mut frame_bytes = Vec::new();
    frame.read_to_end(&mut frame_bytes).unwrap();

    let mut image_one = Canvas::new(314, 524);
    image_one.draw_image(&buf, (6, 4));
    // 301, 465
    image_one.draw_image(&frame_bytes, (0, 0));
    // 314, 524

    let mut image_two = Canvas::new(314, 524);
    image_two.draw_image(&buf, (6, 4));
    image_two.draw_image(&frame_bytes, (0, 0));

    let mut image_three = Canvas::new(314, 524);
    image_three.draw_image(&buf, (6, 4));
    image_three.draw_image(&frame_bytes, (0, 0));

    let mut canvas = Canvas::new(1_008, 524);
    canvas.draw_image(image_one.data().as_bytes(), (1, 1));
    canvas.draw_image(image_two.data().as_bytes(), (347, 1));
    canvas.draw_image(image_three.data().as_bytes(), (692, 1));

    let d = canvas.data();
    // let name = format!("./out/{}.png", Utc::now().timestamp_millis());
    let name = "./out/drop.png";
    let mut file = File::create(name).unwrap();
    let bytes = d.as_bytes();
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

// sync method, fastest cuz its not async
fn thread_job() {
    let mut start1 = Instant::now();
    for i in 0..100 {
        let ins = Instant::now();
        if i == 0 {
            start1 = Instant::now();
        }
        thread::spawn(|| {
            let mut canvas = Canvas::new(2560, 1280);
            canvas.scale(1.2, 1.2);
            canvas.move_to(36.0, 48.0);
            canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
            canvas.translate(10.0, 10.0);
            canvas.set_line_width(20.0);
            canvas.stroke();
            canvas.save();
            canvas.move_to(30.0, 90.0);
            canvas.line_to(110.0, 20.0);
            canvas.line_to(240.0, 130.0);
            canvas.line_to(60.0, 130.0);
            canvas.line_to(190.0, 20.0);
            canvas.line_to(270.0, 90.0);
            canvas.fill();
            let d = canvas.data();
            let name = format!("./out/{}.png", Utc::now().timestamp_millis());
            let mut file = File::create(name).unwrap();
            let bytes = d.as_bytes();
            file.write_all(bytes).unwrap();
        });
        let duration: Duration = ins.elapsed();
        println!("1 Image done: {:?}", duration);
        if i == 99 {
            let duration: Duration = start1.elapsed();
            println!("Loop 1 is: {:?} time: {}", duration, Utc::now().timestamp());
        }
    }
}

// async method one using multi-threaded runtime
async fn runtime_thread_job(mongo_client: &Mongo, rt: &runtime::Runtime) {
    let mut start1 = Instant::now();

    for i in 0..100 {
        let ins = Instant::now();
        if i == 0 {
            start1 = Instant::now();
        }
        let client_ref = mongo_client.get_client().clone();

        let handle = rt.spawn(async move {
            let collection = client_ref
                .database("Sofi")
                .collection::<Document>("character_cards");
            let _doc = mongo::functions::get_random_cards(collection).await;
            // let mut canvas = Canvas::new(2560, 1280);
            // canvas.scale(1.2, 1.2);
            // canvas.move_to(36.0, 48.0);
            // canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
            // canvas.translate(10.0, 10.0);
            // canvas.set_line_width(20.0);
            // canvas.stroke();
            // canvas.save();
            // canvas.move_to(30.0, 90.0);
            // canvas.line_to(110.0, 20.0);
            // canvas.line_to(240.0, 130.0);
            // canvas.line_to(60.0, 130.0);
            // canvas.line_to(190.0, 20.0);
            // canvas.line_to(270.0, 90.0);
            // canvas.fill();
            // let d = canvas.data();
            // let name = format!("./out/{}.png",
            // Utc::now().timestamp_millis()); let mut file =
            // File::create(name).unwrap(); let bytes =
            // d.as_bytes(); file.write_all(bytes).unwrap();
        });

        let _ = tokio::time::timeout(Duration::from_secs(5), handle).await;
        let duration: Duration = ins.elapsed();
        println!("1 Image done: {:?}", duration);
        if i == 99 {
            let duration: Duration = start1.elapsed();
            println!("Loop 1 is: {:?} time: {}", duration, Utc::now().timestamp());
        }
    }
}

// async method two using tokio spawn only
async fn tokio_thread_job(mongo_client: &Mongo) {
    let mut start1 = Instant::now();
    // slower since it requires windows os to spawn
    for i in 0..100 {
        let ins = Instant::now();
        if i == 0 {
            start1 = Instant::now();
        }
        let client_ref = mongo_client.get_client().clone();

        let handle = tokio::spawn(async move {
            let collection = client_ref
                .database("Sofi")
                .collection::<Document>("character_cards");
            let _doc = mongo::functions::get_random_cards(collection).await;
            let mut canvas = Canvas::new(2560, 1280);
            canvas.scale(1.2, 1.2);
            canvas.move_to(36.0, 48.0);
            canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
            canvas.translate(10.0, 10.0);
            canvas.set_line_width(20.0);
            canvas.stroke();
            canvas.save();
            canvas.move_to(30.0, 90.0);
            canvas.line_to(110.0, 20.0);
            canvas.line_to(240.0, 130.0);
            canvas.line_to(60.0, 130.0);
            canvas.line_to(190.0, 20.0);
            canvas.line_to(270.0, 90.0);
            canvas.fill();
            let d = canvas.data();
            let name = format!("./out/{}.png", Utc::now().timestamp_millis());
            let mut file = File::create(name).unwrap();
            let bytes = d.as_bytes();
            file.write_all(bytes).unwrap();
        });

        let _ = tokio::time::timeout(Duration::from_secs(5), handle).await;
        let duration: Duration = ins.elapsed();
        println!("2 Image done: {:?}", duration);
        if i == 99 {
            let duration: Duration = start1.elapsed();
            println!("Loop 1 is: {:?} time: {}", duration, Utc::now().timestamp());
        }
    }
}
