pub mod job;
pub mod queue;

use job::Job;
use queue::JobQueue;
use sdxl::{handle_sdxl_service, SDXLJobRequest};
use tokio::sync::{mpsc, OnceCell}; // Import mpsc for channels

pub struct Worker {
    // No members in Worker
}

impl Worker {
    fn new() -> Self {
        Worker {}
    }

    fn start(&self, mut receiver: mpsc::Receiver<Job>) {
        // Start processing jobs in a separate task
        tokio::spawn(async move {
            println!("Starting job processing...");

            while let Some(job) = receiver.recv().await {
                // Process each job received from the channel
                println!("Processing job: {:?}", job);

                // Here you would call your job processing logic
                // For example:
                do_job(job).await;
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
    async fn new(buffer_size: usize) -> Self {
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

// Public function to add a job
pub async fn add_job(job: Job) {
    // Initialize JOBMANAGER asynchronously if it hasn't been initialized yet
    JOBMANAGER
        .get_or_init(|| async { JobManager::new(100).await })
        .await;

    // Submit the job to JOBMANAGER's queue
    JOBMANAGER.get().unwrap().submit_job(job).await;
}

async fn do_job(job: Job) {
    let req: SDXLJobRequest = job.into();
    handle_sdxl_service(req).await;
}
