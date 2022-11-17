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
    // let redis_connection = caching::Redis::new()?;
    // let mongo = mongo::Mongo::new().await?;
    // redis_connection.drop_all_keys()?;
    // let cards =
    //     mongo::functions::get_all_character_cards(&mongo.get_collection("
    // character_cards")).await; caching::functions::start_db_caching(&
    // redis_connection, cards, "character_cards").await;

    Ok(())
}
