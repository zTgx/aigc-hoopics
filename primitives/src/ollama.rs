use config::CONFIG;
use inspector::Inspector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptRequest {
    pub prompt: String, // 用户输入的prompt
}

impl Inspector for PromptRequest {
    fn inspect(&self) -> Result<bool, String> {
        let checkpoints = &CONFIG.checkpoints;

        // Check prompt length
        if self.prompt.len() > checkpoints.max_length_prompt {
            return Err(format!(
                "Prompt length exceeds maximum of 1500 characters. Current length: {}",
                self.prompt.len()
            ));
        }

        Ok(true)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptResponse {
    pub original_prompt: String, // 用户自己写的prompt
    pub updated_prompt: String,  // 经过ollama处理后的prompt
}
