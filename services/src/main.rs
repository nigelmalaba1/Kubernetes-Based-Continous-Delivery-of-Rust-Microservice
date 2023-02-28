//write an actix microservice with multiple routes
// A. route / that returns a string "Hello, World!"
// B. route /health that returns a 200 status code
// C. route /Gold that returns a string "Gold Status"
// D. route /Silver that returns a string "Silver Status"
// E. route /Bronze that returns a string "Bronze Status"

use actix_web::{web, App, HttpServer};

async fn index() -> &'static str {
    "Hello, World!"
}

async fn health() -> &'static str {
    "200"
}

async fn gold() -> &'static str {
    "Gold Status"
}

async fn silver() -> &'static str {
    "Silver Status"
}

async fn bronze() -> &'static str {
    "Bronze Status"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/Gold", web::get().to(gold))
            .route("/Silver", web::get().to(silver))
            .route("/Bronze", web::get().to(bronze))
    })
    .bind("127.0.0.1:8080") ?
    .run()
    .await
}


