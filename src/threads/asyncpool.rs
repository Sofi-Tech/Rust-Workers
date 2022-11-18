use futures::{
    executor::{ThreadPool, ThreadPoolBuilder},
    Future,
};

pub fn create_new_pool(size: usize, name: String) -> ThreadPool {
    let pool = ThreadPoolBuilder::new()
        .pool_size(size)
        .name_prefix(name)
        .create()
        .unwrap();
    pool
}

// pub fn connect_to_mongo(pool:) -> mongodb::Client {
//     poo
//     let client_uri = "mongodb://localhost:27017/Sofi";
//     let options = mongodb::options::ClientOptions::parse_with_resolver_config(
//         &client_uri,
//         mongodb::options::ResolverConfig::cloudflare(),
//     )
//     .await
//     .unwrap();
//     let client = mongodb::Client::with_options(options).unwrap();
//     client
// }
