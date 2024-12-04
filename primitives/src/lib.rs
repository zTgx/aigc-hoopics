pub mod health;
pub mod job_req;
pub mod job_status;
pub mod ollama;
pub mod sdxl;

use job_req::JobParams;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    id: String,
    pub params: JobParams,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[repr(u8)]
pub enum JobType {
    Txt2Img = 0,
    Img2Img = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[repr(u8)]
pub enum ModelType {
    SDXL = 0,
    FLUX = 1,
}

impl ToString for JobType {
    fn to_string(&self) -> String {
        match self {
            JobType::Txt2Img => "txt2img".to_string(),
            JobType::Img2Img => "img2img".to_string(),
        }
    }
}

impl ToString for ModelType {
    fn to_string(&self) -> String {
        match self {
            ModelType::SDXL => "SDXL".to_string(),
            ModelType::FLUX => "FLUX".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_job_type() {
        let json_data_txt2img = r#"{ "job_type": 0 }"#;
        let json_data_img2img = r#"{ "job_type": 1 }"#;
    
        #[derive(Debug, Serialize, Deserialize)]
        struct JobParamsWrapper {
            job_type: JobType,
        }
    
        // Deserialize JobType from JSON
        let job_params_txt2img: JobParamsWrapper = serde_json::from_str(json_data_txt2img).expect("Failed to deserialize");
        let job_params_img2img: JobParamsWrapper = serde_json::from_str(json_data_img2img).expect("Failed to deserialize");
    
        // Assertions
        assert_eq!(job_params_txt2img.job_type, JobType::Txt2Img);
        assert_eq!(job_params_img2img.job_type, JobType::Img2Img);
    }

    #[test]
    fn test_deserialize_model_type() {
        let json_data_sdxl = r#"{ "model": 0 }"#;
        let json_data_flux = r#"{ "model": 1 }"#;

        // Create a struct to hold model parameters for deserialization
        #[derive(Debug, Serialize, Deserialize)]
        struct ModelParamsWrapper {
            model: ModelType,
        }

        // Deserialize ModelType from JSON
        let model_params_sdxl: ModelParamsWrapper = serde_json::from_str(json_data_sdxl).expect("Failed to deserialize");
        let model_params_flux: ModelParamsWrapper = serde_json::from_str(json_data_flux).expect("Failed to deserialize");

        // Assertions
        assert_eq!(model_params_sdxl.model, ModelType::SDXL);
        assert_eq!(model_params_flux.model, ModelType::FLUX);
    }
}
