use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod login_routes;
mod scan_routes;

pub fn login_route() -> impl HttpServiceFactory {
    println!("Haha");
    web::scope("/login").service(login_routes::login_routes())
}

pub fn scan_route() -> impl HttpServiceFactory {

    web::scope("/scan").service(scan_routes::scan_routes())
}

pub mod common {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct SuccessResponse {
        pub msg: String
    }
}