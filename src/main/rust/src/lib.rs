use std::env;

use actix_web::{App, HttpServer};

use crate::handlers::db::{DB_URL, STORE_INTERNALLY};

pub mod errors;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;

pub async fn start_server() -> std::io::Result<()> {
    let mut args = env::args().into_iter();
    let mut port = "8080".to_string();
    let mut db_url = "127.0.0.1:6379".to_string();

    let mut internal_store = true;
    *STORE_INTERNALLY.lock().unwrap() = true;
    while let Some(arg) = args.next() {
        if arg == "port" {
            port = args.next().expect("wrong port number!");
        }
        if arg == "db-port" {
            db_url = format!("127.0.0.1:{}", args.next().expect("wrong db-port!").trim());
            internal_store = false
        }
        if arg == "db-url" {
            db_url = args.next().expect("wrong db-url!").trim().to_string();
            internal_store = false
        }
    }
    println!("starting server at http://127.0.0.1:{port}");
    match internal_store {
        true => {
            println!("=======< internal storage >=======");
            *STORE_INTERNALLY.lock().unwrap() = true;
        }
        false => {
            println!("connecting to DB {db_url}");
            *STORE_INTERNALLY.lock().unwrap() = false;
            *DB_URL.lock().unwrap() = db_url;
        }
    }

    HttpServer::new(|| App::new().configure(routes::config))
        .workers(8)
        .bind(format!("127.0.0.1:{port}"))?
        .run()
        .await
}
