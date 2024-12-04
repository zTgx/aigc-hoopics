use crate::queue::JobQueue;
use dispatcher::Dispatcher;
use primitives::Job;
use tokio::sync::{mpsc, OnceCell};

pub struct Worker;

impl Worker {
    fn new() -> Self {
        Worker {}
    }

    fn start(&self, mut receiver: mpsc::Receiver<Job>) {
        // Start processing jobs in a separate task
        tokio::spawn(async move {
            println!("Starting job processing...");

            let dispatcher = Dispatcher::new();
            while let Some(job) = receiver.recv().await {
                // Process each job received from the channel
                println!("Processing job: {:?}", job);

                // Here you would call your job processing logic
                // dispatch(job).await;
                dispatcher.dispatch(job).await;
            }
        });
    }
}

// Global manager struct to encapsulate JobQueue and Worker
pub struct JobManager {
    pub job_queue: JobQueue,
    pub worker: Worker,
}

impl JobManager {
    pub async fn new(buffer_size: usize) -> Self {
        let (job_queue, receiver) = JobQueue::new(buffer_size);

        let worker = Worker::new();
        worker.start(receiver); // Start the worker with the receiver

        JobManager { job_queue, worker }
    }

    // Method to submit jobs
    pub async fn submit_job(&self, job: Job) {
        self.job_queue.add_job(job).await; // Submit a job asynchronously
    }
}

// Use OnceCell for asynchronous initialization of JOBMANAGER
pub static JOBMANAGER: OnceCell<JobManager> = OnceCell::const_new();
