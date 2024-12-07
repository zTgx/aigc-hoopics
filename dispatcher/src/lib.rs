pub mod rules;

use primitives::{
    job_status::{JobResult, JobStatusReq},
    Job, ModelType,
};
use sdxl::SDXLClient;

#[derive(Clone)]
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
        if job.model == ModelType::SDXL {
            return self.sdxl.handle(job).await;
        }
    }
}

impl Dispatcher {
    pub async fn check_status(
        &self,
        request: JobStatusReq,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        self.sdxl.handle_job_status(request).await
    }
}
