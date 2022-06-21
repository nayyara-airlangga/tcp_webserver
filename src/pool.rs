use std::{
    error::Error,
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
};

use crate::{task::Task, worker::Worker};

/// A pool of threads
///
/// `ThreadPool` can be used to store a finite amount of worker threads for a certain process
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
}

impl ThreadPool {
    /// Creates a new `ThreadPool` instance
    ///
    /// # Errors
    ///
    /// - `ZeroWorkers` : Happens when trying to create a `ThreadPool` with zero workers
    pub fn new(num: u32) -> Result<ThreadPool, ZeroWorkers> {
        if num > 0 {
            let mut workers = Vec::with_capacity(num as usize);

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..num {
                workers.push(Worker::new(id as usize, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(ZeroWorkers)
        }
    }

    /// Executes the process in which each thread in `ThreadPool` is going to run
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Task::Do(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Terminating all workers...");

        for _ in &mut self.workers {
            self.sender.send(Task::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Worker {} shutting down...", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Error for when the number of workers entered for `ThreadPool` is zero
#[derive(Debug)]
pub struct ZeroWorkers;

impl Display for ZeroWorkers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "num should be greater than zero")
    }
}

impl Error for ZeroWorkers {}
