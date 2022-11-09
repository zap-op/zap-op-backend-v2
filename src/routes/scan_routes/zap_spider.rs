use actix_web::{get, Responder, web};
use actix_web::dev::HttpServiceFactory;

#[get("/")]
async fn get_login() -> impl Responder {
    // println!("Here");
    // web::Json(SuccessResponse {
    //     msg: "success".to_string()
    // })
    "Test"
}

pub fn zap_spider_routes() -> impl HttpServiceFactory {
    web::scope("/").service(get_login)
}
