use std::time::{ Duration, Instant };

use mongodb::{ bson::{ doc, Document }, options::FindOneOptions, Collection };
use rand::Rng;

pub async fn get_random_cards(collection: Collection<Document>) -> (Document, Document, Document) {
    let start = Instant::now();
    let size: u64 = collection.count_documents(None, None).await.unwrap();

    if size < 3 {
        panic!("Not enough cards in the database!");
    }
    let r_numbers = random_numbers(size);
    let opt_one = FindOneOptions::builder().skip(r_numbers[0]).build();
    let opt_two = FindOneOptions::builder().skip(r_numbers[1]).build();
    let opt_three = FindOneOptions::builder().skip(r_numbers[2]).build();

    let query_one = collection.find_one(doc! { "released": true }, opt_one).await;
    let query_two = collection.find_one(doc! { "released": true }, opt_two).await;
    let query_three = collection.find_one(doc! { "released": true }, opt_three).await;

    let duration: Duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    (query_one.unwrap().unwrap(), query_two.unwrap().unwrap(), query_three.unwrap().unwrap())
}

fn random_numbers(max: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = Vec::new();
    while numbers.len() < 3 {
        let num = rng.gen_range(0..max);
        if !numbers.contains(&num) {
            numbers.push(num);
        } else {
            continue;
        }
    }
    // println!("{:?}", numbers);
    numbers
}