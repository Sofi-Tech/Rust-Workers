use rand::Rng;
use mongodb::{ Collection, bson::{ Document, doc }, options::FindOneOptions };
use std::time::{ Duration, Instant };

pub async fn get_random_cards(
    collection: Collection<Document>
) -> Result<(Document, Document, Document), Box<dyn FnOnce() + Send + 'static>> {
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
    Ok((query_one.unwrap().unwrap(), query_two.unwrap().unwrap(), query_three.unwrap().unwrap()))
}

// pub fn generate_print() {
//     struct Rng {
//         min: u32,
//         max: u32,
//     }

//     impl Rng {
//         fn new(min: u32, max: u32) -> Self {
//             Self { min, max }
//         }

//         fn generate(&self) -> u32 {
//             rand::thread_rng().gen_range(self.min..self.max)
//         }
//     }

//     fn get_probability() {
//         struct Ranges {
//             a: Rng,
//             b: Rng,
//             c: Rng,
//             d: Rng,
//             e: Rng,
//             f: Rng,
//         }

//         let ranges = Ranges {
//             a: Rng { min: 0, max: 10 },
//             b: Rng { min: 11, max: 20 },
//             c: Rng { min: 21, max: 30 },
//             d: Rng { min: 31, max: 40 },
//             e: Rng { min: 41, max: 50 },
//             f: Rng { min: 51, max: 60 },
//         };

//         impl Ranges {
//             fn as_array(&self) -> [Rng; 1] {
//                 [Rng { min: self.a.min, max: self.b.max }]
//             }
//         }
//         let num = rand::thread_rng().gen_range(0..8_192 + 1);

//         for i in ranges.as_array().iter() {
//             if num >= i.min && num <= i.max {
//                 i;
//                 println!("{} is in range {} - {}", num, i.min, i.max);
//             }
//         }
//     }
// }

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