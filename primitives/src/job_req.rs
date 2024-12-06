use crate::{JobStyle, JobType, ModelType, Priority};
use config::CONFIG;
// Assuming these are defined elsewhere in your crate
use inspector::Inspector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobParams {
    pub prompt: String,
    pub negative_prompt: String,
    pub job_type: JobType,
    pub img_link: String,
    pub priority: Priority,
    pub description: String,
    pub job_style: JobStyle,
    pub model: ModelType,
    pub width: u16,
    pub height: u16,
    pub rewrite_prompt: bool,
}

impl Inspector for JobParams {
    fn inspect(&self) -> Result<bool, String> {
        let checkpoints = &CONFIG.checkpoints;

        // Check prompt length
        if self.prompt.len() > checkpoints.max_length_prompt {
            return Err(format!(
                "Prompt length exceeds maximum of 1500 characters. Current length: {}",
                self.prompt.len()
            ));
        }

        // Check negative prompt length
        if self.negative_prompt.len() > checkpoints.max_length_negative_prompt {
            return Err(format!(
                "Negative prompt length exceeds maximum of 200 characters. Current length: {}",
                self.negative_prompt.len()
            ));
        }

        // Check description length
        if self.description.len() > checkpoints.max_length_description {
            return Err(format!(
                "Description length exceeds maximum of 200 characters. Current length: {}",
                self.description.len()
            ));
        }

        // Check img_link length
        if self.img_link.len() > checkpoints.max_length_img_link {
            return Err(format!(
                "Image link length exceeds maximum of 256 characters. Current length: {}",
                self.img_link.len()
            ));
        }

        // Check image width
        if self.width < checkpoints.min_image_width || self.width > checkpoints.max_image_width {
            return Err(format!(
                "Image width must be between 1 and 1024. Current width: {}",
                self.width
            ));
        }

        // Check image height
        if self.height < checkpoints.min_image_height || self.height > checkpoints.max_image_height
        {
            return Err(format!(
                "Image height must be between 1 and 1024. Current height: {}",
                self.height
            ));
        }

        match self.job_type {
            JobType::Txt2Img => {
                if !self.img_link.is_empty() {
                    return Err("img_link should be empty for Txt2Img jobs".to_string());
                }
            }
            JobType::Img2Img => {
                if self.img_link.is_empty() {
                    return Err("img_link is required for Img2Img jobs".to_string());
                }
            }
        }

        // If all checks pass, return Ok(true)
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
            "job_style": 0,
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
        assert_eq!(job_params.priority, Priority::Low);
        assert_eq!(job_params.description, "A job for generating images.");
        assert_eq!(job_params.job_style, JobStyle::Normal);
        assert_eq!(job_params.model, ModelType::SDXL); // Ensure this matches the enum or struct definition
        assert_eq!(job_params.width, 512);
        assert_eq!(job_params.height, 512);
        assert!(job_params.rewrite_prompt);
    }
}
