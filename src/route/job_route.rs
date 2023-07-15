
use crate::{
    jwt_auth,
    model::job_model::Job,
    route::user_route::find_user_by_id,
    schema::job_schema::{CreateJobPosting, QueryParam},
    AppState,
};

use actix_web::{
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/job")]
async fn create_job_posting(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
    body: web::Json<CreateJobPosting>,
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
    

    let query_result = sqlx::query_as!(
        Job,
        "INSERT INTO jobs (title, company_name, city, country, salary, description) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        body.title.to_string(),
        body.company_name.to_string(),
        body.city.to_string(),
        body.country.to_string(),
        body.salary.to_string(),
        body.description.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result{
        Ok(job)=>{
            let create_job_response = serde_json::json!({
                "status":"Success",
                "data": serde_json::json!({
                    "id": job.id
                })
            });
            return HttpResponse::Ok().json(create_job_response);
        },
        Err(e)=>{
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "status":"Error",
                    "message": format!("{:?}", e)
                }));
        }

    }
}

#[get("/jobs")]
async fn fetch_job_posting(
    query: web::Query<QueryParam>,
    data: web::Data::<AppState>
)-> impl Responder{

    let mut offset = 0;

    match &query.page{
        None =>  {},
        Some(x)=>{
            offset = x.parse().unwrap();
        }
    };

    offset = offset * 10;

    let jobs = sqlx::query_as!(Job, "SELECT * FROM jobs ORDER BY RANDOM() LIMIT 10 OFFSET $1", offset)
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "status": "Success",
        "message": "Jobs fetched",
        "data": jobs
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/job/{job_id}")]
async fn find_job_by_id(
    data: web::Data::<AppState>,
    params: web::Path<String>
)-> impl Responder{

    let job_id = params.into_inner();
    let job_uuid: uuid::Uuid = Uuid::parse_str(&job_id).unwrap();

    let jobs = sqlx::query_as!(
        Job,
        "SELECT * FROM jobs WHERE id = $1",
        job_uuid
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let json_responder = serde_json::json!({
        "status": "Success",
        "message": "Jobs fetched",
        "data": jobs
    });

    HttpResponse::Ok().json(json_responder)
}