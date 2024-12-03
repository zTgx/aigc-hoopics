use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::h_jobs;
use diesel::sql_types::{BigInt, Varchar, Nullable, Timestamp};

#[derive(QueryableByName, Queryable, Insertable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = h_jobs)] // Ensure this matches your schema definition
pub struct HJobs {
    #[diesel(sql_type = BigInt)]
    pub id: i64,

    #[diesel(sql_type = Varchar)]
    pub api_key: String,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub description: Option<String>,

    #[diesel(sql_type = Varchar)]
    pub job_type: String,

    #[diesel(sql_type = Varchar)]
    pub prompt: String,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub img_link: Option<String>,

    #[diesel(sql_type = Varchar)]
    pub module: String,

    #[diesel(sql_type = Varchar)]
    pub job_id: String,

    #[diesel(sql_type = Varchar)]
    pub job_status: String,

    #[diesel(sql_type = Varchar)]
    pub job_processor: String,

    #[diesel(sql_type = i32)] // Assuming job_priority is an integer
    pub job_priority: i32,

    #[diesel(sql_type = Timestamp)]
    pub created_time: chrono::NaiveDateTime,

    #[diesel(sql_type = Varchar)]
    pub job_style: String,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub negative_prompt: Option<String>,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub results: Option<String>,

    #[diesel(sql_type = Nullable<Timestamp>)]
    pub updated_time: Option<chrono::NaiveDateTime>,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub new_prompt: Option<String>,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub reason: Option<String>,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub aws_results: Option<String>,

    #[diesel(sql_type = Nullable<i32>)] // Assuming prompt_score is an integer
    pub prompt_score: Option<i32>,

    #[diesel(sql_type = Nullable<Varchar>)]
    pub resolution: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}