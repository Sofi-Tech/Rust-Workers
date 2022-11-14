mod mongo;
mod threads;

use std::{error::Error, time::Duration};

use mongo::Mongo;
use threads::ThreadPool;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = ThreadPool::new(10);

    let mongo_client: Mongo = Mongo::new().await.unwrap();
    let collection = mongo_client.get_collection("character_cards");
    let _cards = mongo::functions::get_random_cards(collection).await;
    for _i in 0..8000 {
        pool.execute(|| {
            fibonacci(5);
        });
    }
    sleep(Duration::from_millis(1000 * 60)).await;
    println!("100 ms have elapsed");
    // println!("{:?}", _cards);
    Ok(())
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
