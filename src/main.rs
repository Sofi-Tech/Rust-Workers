#![allow(dead_code)]
mod canvas;
mod mongo;
mod redis;

use std::{
    error::Error,
    time::{Duration, Instant},
};

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

    // println!("{:?}", a.as_bytes());
    // let cards =
    //     mongo::functions::get_all_character_cards(&mongo.get_collection("
    // character_cards")).await; redis::functions::start_db_caching(&
    // redis_connection, cards, "character_cards").await;

    Ok(())
}
