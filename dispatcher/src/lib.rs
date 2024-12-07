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
        // println!(">>> check status, sorted job ids: {:#?}", sorted_job_ids);

        let mut results: Vec<JobResult> = vec![];
        for (model_key, job_ids) in &sorted_job_ids {
            match model_key.to_ascii_uppercase().as_str() {
                "SDXL" => {
                    let result = self.check_sdxl_status(job_ids.clone()).await?;
                    results.extend(result);
                }
                "FLUX" => {
                    let result = self.check_flux_status(job_ids.clone()).await?;
                    results.extend(result);
                }
                _ => continue, // Skip unknown model types
            };
        }

        Ok(results)
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
