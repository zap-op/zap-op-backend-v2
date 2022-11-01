use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod login_routes;
mod scan_routes;

pub fn login_route() -> impl HttpServiceFactory {
    web::scope("/login").service(login_routes::post_login)
}

pub fn scan_route() -> impl HttpServiceFactory {
    web::scope("/scan").service(scan_routes::scan_routes())
}

pub mod common {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct GeneralResponse {
        pub msg: &'static str
    }
}