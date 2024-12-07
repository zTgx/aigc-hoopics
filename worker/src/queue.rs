use primitives::job_req::JobParams;
use tokio::sync::mpsc; // Use Tokio's async Mutex and mpsc channel

pub struct JobQueue {
    pub sender: mpsc::Sender<JobParams>, // Channel sender for submitting jobs
}

impl JobQueue {
    pub fn new(buffer_size: usize) -> (Self, mpsc::Receiver<JobParams>) {
        let (sender, receiver) = mpsc::channel(buffer_size); // Create a channel
        let job_queue = JobQueue { sender };

        (job_queue, receiver) // Return both the job queue and receiver
    }

    // Submit a job to the queue
    pub async fn add_job(&self, param: JobParams) {
        let _ = self.sender.send(param).await; // Send job to channel
    }
}
