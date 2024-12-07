pub mod rules;

use flux::FLUXClient;
use primitives::{
    job_status::{JobResult, JobStatusReq},
    Job, ModelType,
};
use sdxl::SDXLClient;

#[derive(Clone)]
pub struct Dispatcher {
    sdxl: SDXLClient,
    flux: FLUXClient,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            sdxl: SDXLClient::new(),
            flux: FLUXClient::new(),
        }
    }
}

impl Dispatcher {
    pub async fn dispatch(&self, job: Job) -> Result<reqwest::Response, reqwest::Error> {
        match job.model {
            ModelType::SDXL => {
                return self.sdxl.handle(job).await;
            }
            ModelType::FLUX => {
                return self.flux.handle(job).await;
            }
        }
    }

    pub async fn check_status(
        &self,
        request: JobStatusReq,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        let sorted_job_ids = rules::check_status_rules(&request).await.unwrap();

        for (model_key, job_ids) in sorted_job_ids {
            let model_type = match model_key.to_ascii_uppercase().as_str() {
                "SDXL" => ModelType::SDXL,
                "FLUX" => ModelType::FLUX,
                _ => continue, // Skip unknown model types
            };

            return self.check_model_status(&model_type, job_ids).await;
        }

        Ok(vec![])
    }
}

impl Dispatcher {
    // Define a helper function to check status based on model type
    async fn check_model_status(
        &self,
        model_type: &ModelType,
        job_ids: Vec<String>,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        match model_type {
            ModelType::SDXL => self.check_sdxl_status(job_ids).await,
            ModelType::FLUX => self.check_flux_status(job_ids).await,
        }
    }
}

impl Dispatcher {
    async fn check_sdxl_status(
        &self,
        job_ids: Vec<String>,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        self.sdxl.handle_job_status(job_ids.into()).await
    }
}

impl Dispatcher {
    async fn check_flux_status(
        &self,
        job_ids: Vec<String>,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        self.flux.handle_job_status(job_ids.into()).await
    }
}
