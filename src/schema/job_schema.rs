use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateJobPosting{
    pub title: String,
    pub company_name:String,
    pub city: String,
    pub country: String,
    pub salary: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryParam{
    pub page: Option<String>,
    pub search_query: Option<String>,
}