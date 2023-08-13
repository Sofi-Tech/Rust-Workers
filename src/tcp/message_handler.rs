use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Job {
    Ping,
    Pong,
    GenerateImage,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub job: Job,
    pub data: String,
}

pub struct IncomingMessage {
    pub id: u64,
    pub receptive: bool,
    pub payload: Payload,
}

pub async fn handle_message(message: IncomingMessage) -> Option<Payload> {
    match message.payload.job {
        Job::Ping => Some(Payload {
            job: Job::Pong,
            data: "pong".to_string(),
        }),
        _ => Some(Payload {
            job: Job::Unknown,
            data: "unknown".to_string(),
        }),
    }
}
