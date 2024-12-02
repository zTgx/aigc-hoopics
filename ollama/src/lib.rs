use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use config::CONFIG;

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    model: String,
    prompt: String,
    stream: bool,
    keep_alive: i32,
}

impl PostData {
    pub fn new(model: &str, prompt: &str) -> Self {
        PostData {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            keep_alive: -1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlamaPromptResp {
    pub context: Vec<i32>, // 或者 Vec<u32>，根据需要选择合适的类型
    pub created_at: String,
    pub done: bool,
    pub done_reason: String,
    pub eval_count: i32,
    pub eval_duration: i64, // 以纳秒为单位，使用 i64 来表示大数字
    pub load_duration: i64, // 以纳秒为单位
    pub model: String,
    pub prompt_eval_count: i32,
    pub prompt_eval_duration: i64, // 以纳秒为单位
    pub response: String,
    pub total_duration: i64, // 以纳秒为单位
}

pub struct Llama {
    client: Client,
}

impl Llama {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }

    pub async fn prompt(&self, prompt: &str) -> Result<String, Box<dyn Error>> {   
        let post_data = PostData::new(&CONFIG.ollama.model, prompt);

        println!("post_data: {:?}", post_data);

        let resp = self.client.post(CONFIG.ollama.url.clone())
            .json(&post_data)
            .send()
            .await?;

        let llama_resp: LlamaPromptResp = resp.json().await?;
        println!("resp: {:?}", llama_resp);

        if !llama_resp.done {
            return Err("Llama response is not done".into());
        }

        Ok(llama_resp.response)
    }
}