pub mod queue;

use lazy_static::lazy_static;
use queue::JobQueue;
use serde::{Deserialize, Serialize};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub params: serde_json::Value, // Store job parameters as JSON
}

pub struct Worker {
    pub queue: JobQueue,
}

pub fn submit_job(job: Job) {
    HWORKER.queue.add_job(job);
}

impl Worker {
    fn new() -> Self {
        Worker {
            queue: JobQueue::new(),
        }
    }

    fn start(&self) {
        let queue_clone = self.queue.queue.clone();

        // Start processing jobs in a separate thread
        thread::spawn(move || {
            let job_queue = JobQueue { queue: queue_clone };
            job_queue.process_jobs();
        });
    }
}

lazy_static! {
    pub static ref HWORKER: Worker = {
        println!("Worker is processing jobs...");

        let worker = Worker::new();
        worker.start();
        worker
    };
}
