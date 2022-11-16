extern crate redis;
pub mod functions;
pub struct Redis {
    client: redis::Client,
}

impl Redis {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = redis::Client::open("redis://127.0.0.1:6379/")?;
        Ok(Self { client })
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        let value: String = redis::cmd("GET").arg(key).query(&mut con)?;
        Ok(value)
    }

    pub fn set(&self, key: &str, value: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query::<String>(&mut con)?;
        Ok(())
    }

    pub fn del(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        redis::cmd("DEL").arg(key).query::<String>(&mut con)?;
        Ok(())
    }

    pub fn keys(&self, pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        let keys: Vec<String> = redis::cmd("KEYS").arg(pattern).query(&mut con)?;
        Ok(keys)
    }

    pub fn exists(&self, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        let exists: bool = redis::cmd("EXISTS").arg(key).query(&mut con)?;
        Ok(exists)
    }

    pub fn increment_value(&self, key: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        let value: i32 = redis::cmd("INCR").arg(key).query(&mut con)?;
        Ok(value)
    }

    pub fn decrement_value(&self, key: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let mut con = self.client.get_connection()?;
        let value: i32 = redis::cmd("DECR").arg(key).query(&mut con)?;
        Ok(value)
    }
}
