pub mod functions;

use std::error::Error;
use mongodb::{ Client, Database, options::{ ClientOptions, ResolverConfig }, Collection };
// use dotenv_codegen::dotenv;
use bson::{ Document };

pub struct Mongo {
    db: Database,
    // client: Client,
}

impl Mongo {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Load the MongoDB connection string from an environment variable:
        let client_uri = "mongodb://localhost:27017/Sofi";
        // A Client is needed to connect to MongoDB:
        // An extra line of code to work around a DNS issue on Windows:
        let options = ClientOptions::parse_with_resolver_config(
            &client_uri,
            ResolverConfig::cloudflare()
        ).await?;

        let client = Client::with_options(options)?;
        let db = client.database("Sofi");

        Ok(Self { db })
    }

    pub fn get_collection(&self, collection: &str) -> Collection<Document> {
        self.db.collection(collection)
    }

    // pub fn get_client(&self) -> &Client {
    //     &self.client
    // }

    // pub fn get_db(&self) -> &Database {
    //     &self.db
    // }
}