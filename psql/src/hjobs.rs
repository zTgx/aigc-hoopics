use crate::engine::Engine;
use chrono::{DateTime, Utc};
use primitives::Job;
use tokio_postgres::Error;

#[derive(Debug)]
pub struct HJob {
    pub api_key: String,
    pub job_id: String,
    pub job_type: String,
    pub job_style: String,
    pub job_priority: String,
    pub job_status: String,
    pub job_processor: String,
    pub job_model: String,
    pub job_description: Option<String>,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub updated_prompt: Option<String>,
    pub img_link: Option<String>,
    pub output_image_url: Option<String>,
    pub resolution: Option<String>,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
}

impl From<Job> for HJob {
    fn from(job: Job) -> Self {
        HJob {
            api_key: job.api_key,
            job_id: job.id,
            job_type: job.job_type.to_string(),
            job_style: job.job_style.to_string(),
            job_priority: job.priority.to_string(),
            job_status: "pending".to_string(), // TOOD: status : Added -> Pending -> Processing -> Success (Failed) -> Canceled
            job_processor: "1".to_string(),    //TODO: dynamic allcate more processor
            job_model: job.model.to_string(),
            job_description: job.description,
            prompt: job.prompt,
            negative_prompt: job.negative_prompt,
            updated_prompt: job.updated_prompt,
            img_link: job.img_link,
            output_image_url: None,
            resolution: Some(format!("{}X{}", job.width, job.height)),
            created_time: Utc::now(),
            updated_time: Some(Utc::now()),
        }
    }
}

impl Engine {
    pub async fn insert_hjob(&mut self, hjob: HJob) -> Result<(), Error> {
        let client = &mut self.client;

        let statement = client
            .prepare(
                "INSERT INTO hjobs (
                api_key, job_id, job_type, job_style, job_priority, job_status, 
                job_processor, job_model, job_description, prompt, negative_prompt, 
                updated_prompt, img_link, output_image_url, resolution, 
                created_time, updated_time
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )",
            )
            .await?;

        client
            .execute(
                &statement,
                &[
                    &hjob.api_key,
                    &hjob.job_id,
                    &hjob.job_type.to_string(),
                    &hjob.job_style.to_string(),
                    &hjob.job_priority.to_string(),
                    &hjob.job_status,
                    &hjob.job_processor,
                    &hjob.job_model.to_string(),
                    &hjob.job_description,
                    &hjob.prompt,
                    &hjob.negative_prompt,
                    &hjob.updated_prompt,
                    &hjob.img_link,
                    &hjob.output_image_url,
                    &hjob.resolution,
                    &hjob.created_time,
                    &hjob.updated_time,
                ],
            )
            .await?;

        Ok(())
    }
}
