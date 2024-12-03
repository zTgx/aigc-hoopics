// @generated automatically by Diesel CLI.

diesel::table! {
    h_jobs (id) {
        id -> Int8,
        #[max_length = 64]
        job_type -> Varchar,
        #[max_length = 2048]
        prompt -> Varchar,
        #[max_length = 255]
        img_link -> Nullable<Varchar>,
        #[max_length = 32]
        module -> Varchar,
        #[max_length = 128]
        job_id -> Varchar,
        #[max_length = 32]
        job_status -> Varchar,
        #[max_length = 128]
        job_processor -> Varchar,
        job_priority -> Int4,
        created_time -> Timestamp,
        #[max_length = 1024]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        api_key -> Varchar,
        #[max_length = 32]
        job_style -> Varchar,
        #[max_length = 1024]
        negative_prompt -> Nullable<Varchar>,
        #[max_length = 1024]
        results -> Nullable<Varchar>,
        updated_time -> Nullable<Timestamp>,
        #[max_length = 2048]
        new_prompt -> Nullable<Varchar>,
        #[max_length = 128]
        reason -> Nullable<Varchar>,
        #[max_length = 1024]
        aws_results -> Nullable<Varchar>,
        prompt_score -> Nullable<Int4>,
        #[max_length = 64]
        resolution -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    h_jobs,
    posts,
);
