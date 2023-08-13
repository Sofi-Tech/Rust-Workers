use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
    Collection, Cursor,
};
use rand::Rng;
use tokio::try_join;

pub async fn get_random_cards(collection: Collection<Document>) -> (Document, Document, Document) {
    //     let now = Instant::now();
    let size: u64 = collection.count_documents(None, None).await.unwrap();
    // let size: u64 = 38579;
    assert!(size >= 3, "Not enough cards in the database!");
    let r_numbers = random_numbers(size);
    let (one, two, three) = get_three_cards(&collection, r_numbers).await;
    // println!("get_random_cards took: {:?}", now.elapsed());
    (one, two, three)
}

pub async fn get_all_character_cards(collection: &Collection<Document>) -> Cursor<Document> {
    collection.find(None, None).await.unwrap()
}

pub async fn get_three_cards(
    collection: &Collection<Document>,
    r_numbers: Vec<u64>,
) -> (Document, Document, Document) {
    let opt_one = FindOneOptions::builder().skip(r_numbers[0]).build();
    let opt_two = FindOneOptions::builder().skip(r_numbers[1]).build();
    let opt_three = FindOneOptions::builder().skip(r_numbers[2]).build();
    let query_one = collection.find_one(doc! { "released": true }, opt_one);
    let query_two = collection.find_one(doc! { "released": true }, opt_two);
    let query_three = collection.find_one(doc! { "released": true }, opt_three);
    let (one, two, three) = try_join!(query_one, query_two, query_three).unwrap();
    (one.unwrap(), two.unwrap(), three.unwrap())
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
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_tree_unique_random_numbers() {
        // using smaller range to test uniqueness
        let numbers = random_numbers(5);
        assert_eq!(numbers.len(), 3);
        assert_ne!(numbers[0], numbers[1]);
        assert_ne!(numbers[0], numbers[2]);
        assert_ne!(numbers[1], numbers[2]);
    }
}
