use actix_web::{Scope, web};

mod zap_spider;

pub fn scan_routes() -> Scope {
    web::scope("/zap-spider").service(zap_spider::zap_spider_routes())
}
