use actix_web::dev::HttpServiceFactory;
use actix_web::{web, get, Responder};
use crate::routes::common::SuccessResponse;

#[get("/")]
async fn get_login() -> impl Responder {
    println!("Here");
    web::Json(SuccessResponse {
        msg: "success".to_string()
    })
}

pub fn zap_spider_routes() -> impl HttpServiceFactory {
    web::scope("/").service(get_login)
}
