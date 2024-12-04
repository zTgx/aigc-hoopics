use serde::{Deserialize, Serialize};

use crate::Job;

#[derive(Debug, Serialize, Deserialize)]
pub struct SDXLJobRequest {
    pub prompt: String,
    pub job_id: String,
    pub style: String,
    pub model_type: String,
    pub width: u32,
    pub height: u32,
}

impl From<Job> for SDXLJobRequest {
    fn from(item: Job) -> Self {
        SDXLJobRequest {
            prompt: item.params.prompt,
            job_id: item.id,
            style: item.params.job_style,
            model_type: item.params.model.to_string(),
            width: item.params.width,
            height: item.params.height,
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

impl From<Job> for Img2ImgRequest {
    fn from(item: Job) -> Self {
        Img2ImgRequest {
            prompt: item.params.prompt,
            job_id: item.id,
            style: item.params.job_style,
            model_type: item.params.model.to_string(),
            img_url: item.params.img_link,
        }
    }
}
