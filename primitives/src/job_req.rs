use crate::{Job, JobType, ModelType}; // Assuming these are defined elsewhere in your crate
use inspector::Inspector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobParams {
    pub prompt: String,
    pub negative_prompt: String,
    pub job_type: JobType,
    pub img_link: String,
    pub priority: i32, // Assuming priority is an integer
    pub description: String,
    pub job_style: String,
    pub model: ModelType, // Assuming ModelType is defined as an enum or struct
    pub width: u32,       // Assuming width is an unsigned integer
    pub height: u32,      // Assuming height is an unsigned integer
    pub rewrite_prompt: bool,
}

impl From<JobParams> for Job {
    fn from(params: JobParams) -> Self {
        Job {
            id: Uuid::new_v4().to_string(),
            params,
        }
    }
}

impl Inspector for JobParams {
    fn inspect(&self) -> Result<bool, String> {
        // Implement your inspection logic here if needed
        Ok(true)
    }
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_job_params() {
        let json_data = r#"
        {
            "prompt": "Generate an image of a sunset",
            "negative_prompt": "blurry",
            "job_type": 0,
            "img_link": "http://example.com/image.png",
            "priority": 1,
            "description": "A job for generating images.",
            "job_style": "realistic",
            "model": 0,
            "width": 512,
            "height": 512,
            "rewrite_prompt": true
        }
        "#;

        // Deserialize JSON into JobParams
        let job_params: JobParams = serde_json::from_str(json_data).expect("Failed to deserialize");

        // Assertions to verify the deserialized values
        assert_eq!(job_params.prompt, "Generate an image of a sunset");
        assert_eq!(job_params.negative_prompt, "blurry");
        assert_eq!(job_params.job_type, JobType::Txt2Img); // Ensure this matches the enum definition
        assert_eq!(job_params.img_link, "http://example.com/image.png");
        assert_eq!(job_params.priority, 1);
        assert_eq!(job_params.description, "A job for generating images.");
        assert_eq!(job_params.job_style, "realistic");
        assert_eq!(job_params.model, ModelType::SDXL); // Ensure this matches the enum or struct definition
        assert_eq!(job_params.width, 512);
        assert_eq!(job_params.height, 512);
        assert!(job_params.rewrite_prompt);
    }
}
