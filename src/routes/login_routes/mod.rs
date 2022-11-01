use actix_web::{post, web, Responder};
use crate::routes::common::GeneralResponse;

#[post("")]
pub async fn post_login() -> impl Responder {
    web::Json(GeneralResponse {
        msg: "success"
    })
}
