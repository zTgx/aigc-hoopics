use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::job::Job;

pub struct JobQueue {
    pub queue: Arc<Mutex<VecDeque<Job>>>,
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn add_job(&self, job: Job) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(job);
    }

    pub fn process_jobs(&self) {
        loop {
            let job_opt = {
                let mut queue = self.queue.lock().unwrap();
                queue.pop_front()
            };

            if let Some(job) = job_opt {
                // Simulate processing the job
                println!("Processing job: {:?}", job);
                // thread::sleep(Duration::from_secs(30)); // Simulate work

                do_job(job);

                // Here you would write the result to the database
                // For example:
                // self.write_result_to_db(job.id.clone(), "success", None);
            } else {
                // Sleep briefly if no jobs are available to prevent busy waiting
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

fn do_job(job: Job) {}
