use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use crate::handlers::imdb_handler;

pub async fn titles(mut query: web::Query<HashMap<String, String>>) -> impl Responder {
    let size = query
        .remove("size")
        .unwrap_or("10".to_string())
        .parse()
        .unwrap();
    let page = query
        .remove("page")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();
    let same_crew_and_alive: bool = query
        .remove("sameWriterAndDirectorAndIsAlive")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap();

    let response = if same_crew_and_alive {
        imdb_handler::titles_with_same_crew_and_alive(size, page)
    } else {
        let actor1 = query.remove("actor1").unwrap_or(String::new());
        let actor2 = query.remove("actor2").unwrap_or(String::new());
        if actor1.is_empty() || actor2.is_empty() {
            return HttpResponse::BadRequest().body("actor1 and actor2 could not be empty.");
        }
        imdb_handler::common_titles(actor1, actor2, size, page)
    };

    HttpResponse::Ok().json(response)
}

pub async fn rating_by_genre(mut query: web::Query<HashMap<String, String>>) -> impl Responder {
    let size = query
        .remove("size")
        .unwrap_or("10".to_string())
        .parse()
        .unwrap();
    let page = query
        .remove("page")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();

    let genre = query.remove("genre").unwrap();

    let response = imdb_handler::rating_by_genre(genre, size, page);

    HttpResponse::Ok().json(response)
}
