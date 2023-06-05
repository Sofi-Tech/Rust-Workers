use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Job {
//     Ping,
//     Pong,
//     GenerateImage,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    // pub job: Job,
    pub payload: String,
}

pub struct IncomingMessage {
    pub id: u64,
    pub receptive: bool,
    pub data: Payload,
}

pub async fn handle_message(message: IncomingMessage) -> Option<Payload> {
    match message.data.payload.as_str() {
        "ping" => Some(Payload {
            // job: Job::Pong,
            payload: "pong".to_string(),
        }),
        _ => Some(Payload {
            // job: Job::Pong,
            payload: "unknown".to_string(),
        }),
    }
}
