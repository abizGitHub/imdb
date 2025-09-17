use actix_web::{web, HttpResponse, Responder};

use crate::{handlers::echo_handler, models::messages::EchoRequest};

pub async fn echo(req_body: web::Json<EchoRequest>) -> impl Responder {
    let response = echo_handler::process_echo(&req_body);
    HttpResponse::Ok().json(response)
}
