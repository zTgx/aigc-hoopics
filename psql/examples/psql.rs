use psql::{db::establish_connection, insert_job, models::HJobs};

fn main() {
    let pool = establish_connection();
    let mut conn = pool
        .get()
        .expect("Failed to get a connection from the pool");

    let new_job = HJobs {
        id: 1, // Assuming id is auto-incremented by the database
        api_key: "example_api_key".to_string(),
        description: Some("Job description".to_string()),
        job_type: "TypeA".to_string(),
        prompt: "This is a prompt.".to_string(),
        img_link: None,
        module: "Module1".to_string(),
        job_id: "job_123".to_string(),
        job_status: "Pending".to_string(),
        job_processor: "Processor1".to_string(),
        job_priority: 1,
        created_time: chrono::NaiveDateTime::from_timestamp(0, 0),
        job_style: "StyleA".to_string(),
        negative_prompt: None,
        results: None,
        updated_time: None,
        new_prompt: None,
        reason: None,
        aws_results: None,
        prompt_score: None,
        resolution: None,
    };

    match insert_job(&mut conn, &new_job) {
        Ok(rows_inserted) => println!("Inserted {} rows", rows_inserted),
        Err(err) => eprintln!("Error inserting job: {:?}", err),
    }
}
