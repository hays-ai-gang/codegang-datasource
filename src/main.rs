mod handlers;
mod model;
mod storage;

use actix_web::{web, App, HttpServer};
use storage::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data_file =
        std::env::var("DATA_FILE").unwrap_or_else(|_| "codegang-datasource.json".to_string());
    let state = web::Data::new(AppState::new(data_file));

    println!("Starting codegang-datasource on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // Full datasource
            .route("/api/datasource", web::get().to(handlers::datasource::get))
            .route("/api/datasource", web::put().to(handlers::datasource::replace))
            // Services
            .route("/api/services", web::get().to(handlers::services::list))
            .route("/api/services", web::post().to(handlers::services::upsert))
            .route("/api/services/{name}", web::get().to(handlers::services::get))
            .route("/api/services/{name}", web::delete().to(handlers::services::delete))
            // Queue contracts
            .route("/api/queue-contracts", web::get().to(handlers::queue::list))
            .route("/api/queue-contracts", web::post().to(handlers::queue::upsert))
            .route("/api/queue-contracts/{topic}", web::get().to(handlers::queue::get))
            .route("/api/queue-contracts/{topic}", web::delete().to(handlers::queue::delete))
            // NoSQL contracts
            .route("/api/nosql-contracts", web::get().to(handlers::nosql::list))
            .route("/api/nosql-contracts", web::post().to(handlers::nosql::upsert))
            .route("/api/nosql-contracts/{entity}", web::get().to(handlers::nosql::get))
            .route("/api/nosql-contracts/{entity}", web::delete().to(handlers::nosql::delete))
            // Proto contracts
            .route("/api/proto-contracts", web::get().to(handlers::proto::list))
            .route("/api/proto-contracts", web::post().to(handlers::proto::upsert))
            .route("/api/proto-contracts/{name}", web::get().to(handlers::proto::get))
            .route("/api/proto-contracts/{name}", web::delete().to(handlers::proto::delete))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
