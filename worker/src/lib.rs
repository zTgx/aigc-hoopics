use bee::{JobManager, JOBMANAGER};
use primitives::Job;

pub mod bee;
pub mod queue;

// Public function to add a job
pub async fn add_job(job: Job) {
    // Initialize JOBMANAGER asynchronously if it hasn't been initialized yet
    JOBMANAGER
        .get_or_init(|| async { JobManager::new(100).await })
        .await;

    // Submit the job to JOBMANAGER's queue
    JOBMANAGER.get().unwrap().submit_job(job).await;
}
