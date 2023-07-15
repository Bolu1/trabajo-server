use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateApplication{
    pub job_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchApplication{
    pub id: uuid::Uuid,
    pub job_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub applicant_id: uuid::Uuid,
}