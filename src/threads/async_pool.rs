use std::{
    future::Future,
    sync::{mpsc, Arc, Mutex},
};

use tokio::task::JoinHandle;

pub struct ThreadPool {
    workers: Vec<AsyncWorker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(AsyncWorker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn async_execute<F>(&self, f: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let job = Box::new(move || {
            tokio::spawn(f);
        });

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.abort();
            }
        }
    }
}

struct AsyncWorker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl AsyncWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> AsyncWorker {
        let thread = tokio::spawn(async move {
            loop {
                let message = receiver.lock().unwrap().recv();
                println!("Worker {} got a job; executing.", id);
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(e) => {
                        println!("{e}");
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        AsyncWorker {
            id,
            thread: Some(thread),
        }
    }
}
