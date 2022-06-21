use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::task::Task;

/// A worker thread which executes processes
pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Create new instance of `Worker`
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv().unwrap();

            match task {
                Task::Do(job) => {
                    println!("Worker {} received a job. Executing...", id);

                    job.call_box();
                }
                Task::Terminate => {
                    println!("Worker {} received orders to terminate. Terminating...", id);

                    break;
                }
            };
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
