// use actix_web::web;
// use crate::{
//     model::user_model::User,
//     schema::user_schema::{LoginUserSchema, RegisterUserSchema},
//     AppState,
// };

// pub async fn create_user(body: web::Json<RegisterUserSchema>, data: web::Data<AppState>, hashed_password: String)->Result<User, sqlx::Error>{
    
//     sqlx::query_as!(
//         User,
//         "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
//         body.first_name.to_string(),
//         body.last_name.to_string(),
//         body.email.to_string().to_lowercase(),
//         hashed_password
//     )
//     .fetch_one(&data.db)
//     .await
// }

// // pub async fn find_user_by_email(data: web::Data<AppState>,  &email: &str)-> Option<User>{

// //     sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", )
// //         .fetch_optional(&data.db)
// //         .await
// //         .unwrap()
// // }