use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;

mod routes;
use routes::{login_route, scan_route};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not found");
    let database = env::var("DATABASE").expect("DATABASE not found");
    let db_client = Client::with_options(
            ClientOptions::parse_with_resolver_config(&mongodb_uri, ResolverConfig::cloudflare())
            .await
            .expect("Failed to config mongodb client")
    ).expect("Failed to connect mongodb")
    .database(&database);
    println!("Database connected");

    let mut port = 8888;
    if let Ok(v) = env::var("PORT") {
        if let Ok(p) = v.parse::<u16>() {
            port = p;
        }
    }

    let server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db_client.clone()))
        .service(login_route())
        .service(scan_route())
    })
    .bind(("127.0.0.1", port))?
    .run();
    println!("Server ready on port {}", port);

    server.await
}