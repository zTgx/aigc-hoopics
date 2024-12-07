use crate::{queue::JobQueue, re_mapping_job};
use dispatcher::Dispatcher;
use primitives::job_req::JobParams;
use psql::engine::Engine;
use tokio::sync::{mpsc, OnceCell};

pub struct Worker;
impl Worker {
    fn new() -> Self {
        Worker {}
    }

    fn start(&self, mut receiver: mpsc::Receiver<JobParams>) {
        // Start processing jobs in a separate task
        tokio::spawn(async move {
            let dispatcher = Dispatcher::new();
            let mut engine = Engine::new().await;

            while let Some(param) = receiver.recv().await {
                // Process each job received from the channel
                println!("Processing job: {:#?}", param);

                let job = re_mapping_job(&param).await;

                if let Err(e) = engine.save_job(job.clone()).await {
                    eprintln!("Error saving job: {}", e);
                }

                // Here you would call your job processing logic
                if let Err(e) = dispatcher.dispatch(job).await {
                    eprintln!("Error dispatch job: {}", e);
                }
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
    pub async fn submit_job(&self, param: JobParams) {
        self.job_queue.add_job(param).await; // Submit a job asynchronously
    }
}

// Use OnceCell for asynchronous initialization of JOBMANAGER
pub static JOBMANAGER: OnceCell<JobManager> = OnceCell::const_new();
