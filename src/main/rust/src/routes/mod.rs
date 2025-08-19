use actix_web::web;

mod echo;
mod upload;

pub const API_VERSION: &str = "/api/v1";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(API_VERSION)
            .route("/files", web::post().to(upload::upload_file))
            .route("/echo", web::post().to(echo::echo)),
    );
}
