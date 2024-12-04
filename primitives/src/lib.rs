pub mod sdxl;
pub mod job_req;
pub mod job_status;
pub mod ollama;
pub mod health;

use sdxl::SDXLJobRequest;
use job_req::JobParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    id: String,
    params: JobParams,
}

impl From<Job> for SDXLJobRequest {
    fn from(item: Job) -> Self {
        SDXLJobRequest {
            prompt: item.params.prompt,
            job_id: item.id,
            style: item.params.job_style,
            model_type: item.params.model,
            width: item.params.width,
            height: item.params.height,
        }
    }
}
