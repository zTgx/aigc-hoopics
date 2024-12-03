pub mod queue;
pub mod job;

use job::Job;
use lazy_static::lazy_static;
use queue::JobQueue;
use std::thread;
use async_once::AsyncOnce;


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
            AsyncOnce::new(async {
                let job_queue = JobQueue { queue: queue_clone };
                job_queue.process_jobs();    
            });
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
