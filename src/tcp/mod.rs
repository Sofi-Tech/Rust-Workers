use std::error::Error;

use serde::{Deserialize, Serialize};
pub mod functions;

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    payload: String,
}

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::tcp::functions::{create, create_from_id, read};

// TODO: impliment event emitter
pub async fn start_tcp() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    tokio::spawn(async move {
        while let Ok((stream, address)) = listener.accept().await {
            println!("New connection established {}", address);
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    });

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; 1024];

    let response_data = rmp_serde::to_vec_named("server")?;
    let data = create(true, &response_data);
    // let id = read(&data);
    stream.write_all(&data).await?;
    stream.flush().await?;

    let num_bytes = stream.read(&mut buffer).await?;
    if num_bytes != 0 {
        // let id = read(&buffer).id;

        let socket_name: String = rmp_serde::from_slice(&buffer[11..num_bytes])?;

        println!("Connected to: {}", socket_name)
    }

    loop {
        let num_bytes = stream.read(&mut buffer).await?;
        if num_bytes == 0 {
            break;
        }

        let id = read(&buffer).id;

        // Deserialize the received data into a Message struct
        let message: Payload = rmp_serde::from_slice(&buffer[11..num_bytes])?;

        // Process the message or perform any required operations
        println!("Received message from id {}: {:?}", id, message);

        let response = Payload {
            payload: format!("Hello, {}!", message.payload),
        };

        let response_data = rmp_serde::to_vec_named(&response)?;
        let data = create_from_id(id, false, &response_data);
        stream.write_all(&data).await?;
        stream.flush().await?;
    }

    Ok(())
}
