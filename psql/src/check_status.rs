use crate::engine::Engine;
use std::collections::HashMap;
use tokio_postgres::Error;

impl Engine {
    pub async fn sort_jobs_by_model(
        &mut self,
        job_ids: &Vec<String>,
    ) -> Result<HashMap<String, Vec<String>>, Error> {
        // Prepare a SQL query with placeholders for job IDs
        let placeholders: Vec<String> = job_ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("${}", i + 1))
            .collect();

        let query = format!(
            "SELECT job_model, job_id FROM hjobs WHERE job_id IN ({})",
            placeholders.join(", ")
        );

        // Execute the query
        let rows = self
            .client
            .query(
                &query,
                &job_ids
                    .iter()
                    .map(|id| id as &(dyn tokio_postgres::types::ToSql + Sync))
                    .collect::<Vec<&(dyn tokio_postgres::types::ToSql + Sync)>>(),
            )
            .await?;

        // Create a HashMap to categorize results
        let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

        // Iterate over the rows and categorize them
        for row in rows {
            let job_id: String = row.get("job_id");
            let model: String = row.get("job_model");

            result_map
                .entry(model)
                .or_insert_with(Vec::new)
                .push(job_id);
        }

        Ok(result_map)
    }
}
