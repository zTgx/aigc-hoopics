use serde::{Deserialize, Serialize};

use crate::{Job, JobStyle, ModelType};

#[derive(Debug, Serialize, Deserialize)]
pub struct SDXLJobRequest {
    pub prompt: String,
    pub job_id: String,
    pub style: JobStyle,
    pub model_type: ModelType,
    pub width: u16,
    pub height: u16,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Img2ImgRequest {
    prompt: String,
    job_id: String,
    img_url: String,
    style: JobStyle,
    model_type: ModelType,
}

impl From<Job> for Img2ImgRequest {
    fn from(item: Job) -> Self {
        Img2ImgRequest {
            prompt: item.params.prompt,
            job_id: item.id,
            style: item.params.job_style,
            model_type: item.params.model,
            img_url: item.params.img_link,
        }
    }
}
