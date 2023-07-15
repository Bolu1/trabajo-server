pub mod user_route;
pub mod job_route;
use actix_web::web;


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(user_route::register_user_handler)
        .service(user_route::login_user_handler)
        .service(user_route::logout_handler)
        .service(user_route::get_me_handler)
        .service(user_route::register_admin_handler)
        .service(job_route::create_job_posting)
        .service(job_route::search_job_posting)
        .service(job_route::find_job_by_id)
        .service(job_route::fetch_job_posting);

    conf.service(scope);
}