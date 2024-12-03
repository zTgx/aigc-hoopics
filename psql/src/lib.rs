extern crate diesel;

pub mod db;
pub mod models;
pub mod schema;

use diesel::{PgConnection, QueryResult, RunQueryDsl};
use models::HJobs;
use schema::h_jobs;

pub fn insert_job(conn: &mut PgConnection, new_job: &HJobs) -> QueryResult<usize> {
    diesel::insert_into(h_jobs::table)
        .values(new_job)
        .execute(conn)
}

// fn find_job_by_job_id(conn: &mut PgConnection, search_job_id: &str) -> QueryResult<HJobs> {
//     let query = "SELECT * FROM sd_jobs WHERE job_id = $1"; // Use parameterized query to prevent SQL injection

//     let job = diesel::sql_query(query)
//         .bind::<Varchar, _>(search_job_id) // Bind the search_job_id parameter
//         .get_result(conn)?; // Execute the query and get the result

//     Ok(job)
//  }
