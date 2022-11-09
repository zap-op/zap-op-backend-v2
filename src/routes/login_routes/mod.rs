use actix_web::{get, post, Responder, web};

use crate::routes::common_response::GeneralResponse;

#[post("")]
pub async fn post_login() -> impl Responder {
    web::Json(GeneralResponse {
        msg: "success"
    })
}

#[get("/cb")]
pub async fn success_login_cb() -> impl Responder {
    web::Json(GeneralResponse {
        msg: "success"
    })
}
