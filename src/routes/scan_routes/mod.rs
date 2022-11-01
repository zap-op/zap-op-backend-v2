mod zap_spider;

use actix_web::{web, Scope};

pub fn scan_routes() -> Scope {
    web::scope("/zap-spider").service(zap_spider::zap_spider_routes())
}
