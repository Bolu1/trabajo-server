use chrono::prelude::*;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser{
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub resume: String,
    pub is_verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData{
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse{
    pub status: String,
    pub message: String,
    pub data: UserData,
}