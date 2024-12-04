pub mod rules;

use primitives::Job;
use sdxl::SDXLClient;

pub struct Dispatcher {
    sdxl: SDXLClient,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            sdxl: SDXLClient::new(),
        }
    }
}

impl Dispatcher {
    pub async fn dispatch(&self, job: Job) {
        if job.params.model == job.params.model {
            return self.sdxl.handle(job).await;
        }
    }
}
