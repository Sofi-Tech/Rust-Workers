use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
pub mod functions;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex, MutexGuard},
};

use crate::tcp::functions::{create, create_from_id, read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub payload: String,
}

pub struct IncomingMessage {
    pub id: u64,
    pub receptive: bool,
    pub data: Payload,
}

#[derive(Clone, Debug)]
pub struct Server {
    pub name: String,
    pub clients: HashMap<String, Arc<Mutex<TcpStream>>>,
    pub queue: HashMap<u64, mpsc::Sender<Payload>>,
}

pub async fn handle_connection(
    server_mutex: Arc<Mutex<Server>>,
    client_mutex: Arc<Mutex<TcpStream>>,
    handle_message: fn(IncomingMessage) -> Option<Payload>,
) -> Result<(), Box<dyn Error>> {
    let mut server = server_mutex.lock().await;
    let mut client = client_mutex.lock().await;
    let mut buffer = vec![0; 1024];

    let response_data = rmp_serde::to_vec_named(&server.name)?;
    let data = create(true, &response_data);
    client.write_all(&data).await?;
    client.flush().await?;

    let mut name = "".to_string();

    loop {
        let num_bytes = client.read(&mut buffer).await?;
        if num_bytes == 0 {
            break;
        }

        let header = read(&buffer);

        if name.is_empty() {
            // TODO: we might need to verify the id here
            let client_name: String = rmp_serde::from_slice(&buffer[11..num_bytes])?;

            println!("Connected to: {}", client_name);

            name = client_name.clone();

            //TODO: close old one if one with same name found
            server.clients.insert(client_name, client_mutex.clone());
        } else {
            let message: Payload = rmp_serde::from_slice(&buffer[11..num_bytes])?;

            println!("Message: {:?}", message);

            let queue_item = server.queue.get_mut(&header.id);

            match queue_item {
                Some(resolve) => {
                    resolve.send(message).await?;
                }
                None => {
                    let message = IncomingMessage {
                        id: header.id,
                        receptive: header.receptive,
                        data: message,
                    };

                    let data = handle_message(message);

                    if let Some(data) = data {
                        if !header.receptive {
                            return Err("Cannot reply to a non-receptive message".into());
                        }

                        let response_data = rmp_serde::to_vec_named(&data)?;
                        let data = create_from_id(header.id, false, &response_data);
                        client.write_all(&data).await?;
                        client.flush().await?;
                    }
                }
            }
        }
    }

    Ok(())
}

impl Server {
    pub fn new(name: String) -> Server {
        Server {
            name,
            clients: HashMap::new(),
            queue: HashMap::new(),
        }
    }

    pub async fn bind(
        self,
        addr: &str,
        handle_message: fn(IncomingMessage) -> Option<Payload>,
    ) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;

        tokio::spawn(async move {
            while let Ok((client, address)) = listener.accept().await {
                println!("New connection established {}", address);
                let server = Arc::new(Mutex::new(self.clone()));

                tokio::spawn(async move {
                    if let Err(e) =
                        handle_connection(server, Arc::new(Mutex::new(client)), handle_message)
                            .await
                    {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
        });

        Ok(())
    }

    pub async fn send<T>(
        &mut self,
        name: String,
        data: T,
        options: Option<bool>,
    ) -> Result<Option<Payload>, Box<dyn Error>>
    where
        T: Serialize,
    {
        let client = match self.clients.entry(name.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(_) => return Err(format!("No client found with name {}", name).into()),
        };

        let mut client = client.lock().await;

        let response_data = rmp_serde::to_vec_named(&data)?;
        let receptive = options.unwrap_or(true);
        let data = create(receptive, &response_data);

        client.write_all(&data).await?;
        client.flush().await?;

        if !receptive {
            return Ok(None);
        }

        let id = read(&data).id;

        // TODO: add timeout
        let (tx, mut rx) = mpsc::channel(1);

        self.queue.insert(id, tx);

        Ok(Some(rx.recv().await.unwrap()))
    }
}
