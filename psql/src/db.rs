use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use config::CONFIG;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

fn database_url() -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        CONFIG.postgres.user, CONFIG.postgres.password, CONFIG.postgres.host, CONFIG.postgres.port, CONFIG.postgres.dbname
    )
}

pub fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}