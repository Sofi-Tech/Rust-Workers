use reqwest::Client; // 0.10.6
pub struct Request {
    pub client: Client,
}

impl Request {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_buffer(&self, url: &str) -> Vec<u8> {
        let res = self.client.get(url).send().await.unwrap();
        let buffer = res.bytes().await.unwrap().to_vec();
        buffer
    }

    pub fn copy(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

pub async fn fetch_buffer(url: &str) -> Vec<u8> {
    let res = reqwest::get(url).await.unwrap();
    let buffer = res.bytes().await.unwrap().to_vec();
    buffer
}
