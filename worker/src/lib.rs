use bee::{JobManager, JOBMANAGER};
use ollama::Llama;
use primitives::{job_req::JobParams, Job};
use uuid::Uuid;

pub mod bee;
pub mod queue;

// Public function to add a job
pub async fn add_job(param: JobParams) {
    // Initialize JOBMANAGER asynchronously if it hasn't been initialized yet
    JOBMANAGER
        .get_or_init(|| async { JobManager::new(100).await })
        .await;

    // Submit the job to JOBMANAGER's queue
    JOBMANAGER.get().unwrap().submit_job(param).await;
}

async fn re_mapping_job(param: &JobParams) -> Job {
    let param = param.clone();
    let updated_prompt = {
        if param.rewrite_prompt {
            let ollama = Llama::new();
            let prompt = ollama.prompt(&param.prompt).await.unwrap();

            Some(prompt)
        } else {
            None
        }
    };

    Job {
        id: Uuid::new_v4().to_string(),
        api_key: "dev".to_string(), //TODO: need parse x-api-key
        prompt: param.prompt,
        updated_prompt,
        negative_prompt: param.negative_prompt,
        job_type: param.job_type,
        img_link: param.img_link,
        priority: param.priority,
        description: param.description,
        job_style: param.job_style,
        model: param.model,
        width: param.width,
        height: param.height,
        rewrite_prompt: param.rewrite_prompt,
    }
}
