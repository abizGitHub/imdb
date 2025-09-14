use actix_web::{App, HttpServer};

mod handlers;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| App::new().configure(routes::config))
        .workers(8)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
