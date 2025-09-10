use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, web, App, Error, HttpResponse, HttpServer};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(Mutex::new(0));

    HttpServer::new(move || {
        App::new()
            .wrap(CallCounter {
                counter: counter.clone(),
            })
            .wrap(MyLogger)
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, World!") }))
            .route("/count", web::get().to({
                let counter = counter.clone();
                move || {
                    let count = *counter.lock().unwrap();
                    async move { HttpResponse::Ok().body(format!("API call count: {}", count)) }
                }
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
