use crate::{
    jwt_auth,
    route::user_route::find_user_by_id,
    schema::{application_schema::{CreateApplication, FetchApplication}, job_schema::QueryParam},
    model::application_model::Application,
    AppState,
};

use actix_web::{
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use uuid::Uuid;



#[post("/application")]
async fn create_application(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
    body: web::Json<CreateApplication>,
)-> impl Responder{

    let ext = req.extensions();
    let user_id: &uuid::Uuid = ext.get::<uuid::Uuid>().unwrap();
    let job_id: uuid::Uuid = Uuid::parse_str(&body.job_id).unwrap();

    // check if application exists
    let application_info = fetch_application_by_id(&data, user_id, job_id).await;
    match application_info{
        None =>{}
        Some(application) =>{
            let create_application_response = serde_json::json!({
                "status": "Success",
                "message": "Application submitted",
                "data": serde_json::json!({
                    "id": application.id
                })
            });
            return HttpResponse::Ok().json(create_application_response)
        }
    };

    let query_result = sqlx::query_as!(
        Application,
        "INSERT INTO applications (job_id, user_id) VALUES ($1, $2) RETURNING *",
            job_id,
            user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(application)=>{
            let create_application_response = serde_json::json!({
                "status": "Success",
                "message": "Application submitted",
                "data": serde_json::json!({
                    "id": application.id
                })
            });
            return HttpResponse::Ok().json(create_application_response)
    },
    Err(e)=>{
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({
                "status":"Error",
                "message": format!("{:?}", e)
            }))
    }
    }
}

#[get("/applications")]
async fn fetch_application(
    req: HttpRequest,
    query: web::Query::<QueryParam>,
    data: web::Data::<AppState>,
    _: jwt_auth::JwtMiddleware
)-> impl Responder{

    let ext = req.extensions();
    let user_id: &uuid::Uuid = ext.get::<uuid::Uuid>().unwrap();

    // check if user is an admin
    let user_info = find_user_by_id(&data, user_id).await;
    println!("{:?}", user_info);
    if user_info.role != "Admin"{
        return  HttpResponse::Unauthorized().json(
            serde_json::json!({
                "status":"Error",
                "message": "Unauthorized"
            })
        );
    }

    let mut offset = 0;

    match &query.page{
        None =>  {},
        Some(x)=>{
            offset = x.parse().unwrap();
        }
    };

    offset = offset * 10;

    let applications =  sqlx::query_as!(
        FetchApplication,
        "SELECT applications.id, applications.job_id, applications.created_at, users.first_name, users.last_name, users.id AS applicant_id
         FROM applications 
         INNER JOIN users ON applications.user_id = users.id
        LIMIT 10 OFFSET $1",
        offset
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_responder = serde_json::json!({
        "status": "Success",
        "message": "Applications fetched",
        "data": applications
    });
    HttpResponse::Ok().json(json_responder)
}

#[get("/application/{job_id}")]
async fn fetch_job_application(
    req: HttpRequest,
    params: web::Path<String>,
    data: web::Data::<AppState>,
    _: jwt_auth::JwtMiddleware
)-> impl Responder{

    let ext = req.extensions();
    let user_id: &uuid::Uuid = ext.get::<uuid::Uuid>().unwrap();
    let job_id = params.into_inner();
    let job_uuid: uuid::Uuid = Uuid::parse_str(&job_id).unwrap();

    // check if user is an admin
    let user_info = find_user_by_id(&data, user_id).await;
    println!("{:?}", user_info);
    if user_info.role != "Admin"{
        return  HttpResponse::Unauthorized().json(
            serde_json::json!({
                "status":"Error",
                "message": "Unauthorized"
            })
        );
    }

    let applications =  sqlx::query_as!(
        FetchApplication,
        "SELECT applications.id, applications.job_id, applications.created_at, users.first_name, users.last_name, users.id AS applicant_id
         FROM applications 
         INNER JOIN users ON applications.user_id = users.id
         WHERE applications.job_id = $1",
         job_uuid
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_responder = serde_json::json!({
        "status": "Success",
        "message": "Applications fetched",
        "data": applications
    });
    HttpResponse::Ok().json(json_responder)
}

async fn fetch_application_by_id(
    data: &web::Data<AppState>,
    user_id: &uuid::Uuid,
    job_id: uuid::Uuid
)-> Option<Application>{

    let query_result = sqlx::query_as!(Application, "SELECT * FROM applications WHERE user_id = $1 AND job_id = $2", user_id, job_id)
    .fetch_optional(&data.db)
    .await
    .unwrap();

    query_result
}