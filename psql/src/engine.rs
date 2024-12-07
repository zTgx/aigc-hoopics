use config::CONFIG;
use primitives::{Job, ModelType};
use tokio_postgres::{Client, NoTls};

#[macro_export]
macro_rules! create_connection_postgres_string {
    ($host:expr, $user:expr, $password:expr, $port:expr, $dbname:expr) => {
        format!(
            "host={} user={} password={} port={} dbname={}",
            $host, $user, $password, $port, $dbname
        )
    };
}

fn database_url() -> String {
    CONFIG.postgres.to_string()
}

pub struct Engine {
    pub client: Client,
}

impl Engine {
    pub async fn new() -> Self {
        let (client, connection) = tokio_postgres::connect(&database_url(), NoTls)
            .await
            .unwrap();

        // Spawn the connection on a separate task to handle it asynchronously
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Self { client }
    }

    pub fn fetch_pending_or_processing_job_ids(_model_type: ModelType) -> Vec<String> {
        vec!["cf628bc0-c15f-4966-aab6-f0c3e8bd2b57".to_string()]
    }

    // Save job to hjobs table
    pub async fn save_job(&mut self, job: Job) -> Result<(), tokio_postgres::Error> {
        self.insert_hjob(job.into()).await
    }
}
