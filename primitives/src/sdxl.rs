use crate::Job;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SDXLJobRequest {
    pub job_id: String,
    pub style: String,
    pub model_type: String,
    pub prompt: String,
    pub width: u16,
    pub height: u16,
}

impl SDXLJobRequest {
    pub fn new(job: Job) -> Self {
        Self {
            job_id: job.id,
            style: job.job_style.to_string(),
            model_type: job.model.to_string(),
            prompt: if let Some(updated_prompt) = job.updated_prompt {
                updated_prompt
            } else {
                job.prompt
            },
            width: job.width,
            height: job.height,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Img2ImgRequest {
    prompt: String,
    job_id: String,
    img_url: String,
    style: String,
    model_type: String,
}

impl Img2ImgRequest {
    pub fn new(job: Job) -> Self {
        Self {
            job_id: job.id,
            prompt: if let Some(updated_prompt) = job.updated_prompt {
                updated_prompt
            } else {
                job.prompt
            },
            style: job.job_style.to_string(),
            model_type: job.model.to_string(),
            img_url: job.img_link.expect("img link is empty"),
        }
    }
}
