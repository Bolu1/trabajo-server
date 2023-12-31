
use crate::{
    jwt_auth,
    model::user_model::User,
    schema::user_schema::{LoginUserSchema, RegisterUserSchema, TokenClaims},
    core::helpers::response::FilteredUser,
    AppState,
};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::Row;

fn filter_user_record(user: &User) -> FilteredUser{
    FilteredUser{
        id: user.id.to_string(),
        email: user.email.to_owned(),
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned(),
        resume: user.resume.to_owned(),
        role: user.role.to_owned(),
        is_verified: user.is_verified,
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}

#[post("/auth/user/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder{
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists{
        return HttpResponse::Conflict().json(
            serde_json::json!({
                "status": "Error",
                "message": "Email already in use"
            })
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email.to_string().to_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(user)=>{
            let user_response = serde_json::json!({
                "status": "Success",
                "data": serde_json::json!({
                    "user": filter_user_record(&user)
                })
            });
            return HttpResponse::Ok().json(user_response);
        }
        Err(e)=>{
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "status": "Error",
                    "message": format!("{:?}", e)
                }));
        }
    }
}

#[post("/auth/admin/register")]
async fn register_admin_handler(
    body: web::Json::<RegisterUserSchema>,
    data: web::Data<AppState>
)-> impl Responder{

    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists{
        return HttpResponse::Conflict().json(
            serde_json::json!({
                "status": "Error",
                "message": "Email already in use"
            })
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, email, password, role) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email.to_string().to_lowercase(),
        hashed_password,
        "Admin"
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(user)=>{
            let user_response = serde_json::json!({
                "status": "Success",
                "data": serde_json::json!({
                    "user": filter_user_record(&user)
                })
            });
            return HttpResponse::Ok().json(user_response);
        }
        Err(e)=>{
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "status": "Error",
                    "message": format!("{:?}", e)
                }));
        }
    }
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json::<LoginUserSchema>,
    data: web::Data<AppState>
)-> impl Responder{
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
        .fetch_optional(&data.db)
        .await
        .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user|{
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid{
        return HttpResponse::BadRequest()
            .json(json!({
                "status": "Error",
                "message": "Invalid login details"
            }));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now+Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims{
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60*60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({
            "status": "Success",
            "token": token
        }))
}

#[get("/auth/logout")]
async fn logout_handler(
    _: jwt_auth::JwtMiddleware
)-> impl Responder{
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({
            "status": "Success",
            "message": "Session ended"
        }))
}

#[get("/auth/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder{
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
    .fetch_one(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "status": "Success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });

    HttpResponse::Ok().json(json_response)
}


pub async fn find_user_by_id(
    data: &web::Data<AppState>,
    user_id: &uuid::Uuid
)-> User{
    
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
    .fetch_one(&data.db)
    .await
    .unwrap();
    user
}