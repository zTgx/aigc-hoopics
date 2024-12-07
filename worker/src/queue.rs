use primitives::job_req::JobParams;
use tokio::sync::mpsc;

pub struct JobQueue {
    pub sender: mpsc::Sender<JobParams>, // Channel sender for submitting jobs
}

// The channel will buffer up to the provided number of messages. Once the buffer is full, attempts to send new messages will wait until a message is received from the channel. The provided buffer capacity must be at least 1.
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
