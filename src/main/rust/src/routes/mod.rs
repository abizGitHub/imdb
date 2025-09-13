use actix_web::web;

use crate::handlers::counter_middleware::{self, CallCounter};

mod echo;
mod upload;
mod title_router;

pub const API_VERSION: &str = "/api/v1";

pub fn config(cfg: &mut web::ServiceConfig) {
     cfg.service(
        web::scope(API_VERSION)
            .route("/imdb/titles", web::get().to(title_router::titles)) 
            .route("/imdb/titles/year", web::get().to(title_router::rating_by_genre)) 
            .route("/files", web::post().to(upload::upload_file))
            .route("/echo", web::post().to(echo::echo))
            .route("/count", web::get().to(counter_middleware::get_counter))
            .wrap(CallCounter::new()));
}
