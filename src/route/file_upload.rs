use actix_multipart::{ Multipart };
use actix_web::{Responder, HttpMessage};
use futures_util::{ TryStreamExt as _ };
use mime::{ Mime, IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF, APPLICATION_PDF };
use serde_json::json;
use uuid::Uuid;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use actix_web::{
    HttpResponse,
    HttpRequest,
    web,
    patch,
    http::header::CONTENT_LENGTH };
use crate::{
    jwt_auth,
    model::user_model::User,
    AppState,
};
    


#[patch("/resume")]
async fn upload(
    mut payload: Multipart,
    req: HttpRequest, 
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {

    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
        None => "0".parse().unwrap(),
    };

    let max_file_count: usize = 3;
    let max_file_size: usize = 300_000;
    let legal_filetypes: [Mime; 4] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF, APPLICATION_PDF];
    let mut current_count: usize = 0;
    let dir: &str = "./static/";

    if content_length > max_file_size { 
        return HttpResponse::BadRequest()
        .json(json!({
            "status": "Error",
            "message": "Invalid login details"
        }));
     }


    loop {
        if current_count == max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_filetypes.contains(&filetype.unwrap()) { continue; }

            println!("content_length: {:#?}", content_length);
            println!("{}. picture:", current_count);
            println!("name {}", field.name());

            // In a multipart/form-data body, the HTTP Content-Disposition general header is a header that can be used on the subpart of a multipart body to give information about the field it applies to. The subpart is delimited by the boundary defined in the Content-Type header. Used on the body itself, Content-Disposition has no effect.
            println!("content disposition {}", field.content_disposition()); // &ContentDisposition

            println!("filename {}", field.content_disposition().get_filename().unwrap()); // Option<&str>
            
            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );
            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            println!("{:?} m,", saved_file);

            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            let _ = sqlx::query_as!(
                User,
                "UPDATE users set resume = $1 WHERE id = $2 returning *",
                destination,
                user_id
            )
            .fetch_one(&data.db)
            .await;

        } else { break; }
        current_count += 1;
    }

    let json_response = serde_json::json!({
        "status": "Success",
        "message": "Resume uploaded"
    });
    HttpResponse::Ok().json(json_response)
}

