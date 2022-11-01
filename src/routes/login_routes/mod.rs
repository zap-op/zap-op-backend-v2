use actix_web::{web, get, Responder};
use actix_web::dev::HttpServiceFactory;
use crate::routes::common::SuccessResponse;

#[get("/")]
async fn post_login() -> impl Responder {
    // web::Json(SuccessResponse {
    //     msg: "success".to_string()
    // })
    "Hello"
}

pub fn login_routes() -> impl HttpServiceFactory {
    web::scope("/").service(post_login)
}
