use actix_web::{App, HttpServer};

pub mod errors;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .workers(8)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
