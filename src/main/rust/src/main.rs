use std::env;

use simple_api::{
    handlers::db::{DB_URL, STORE_INTERNALLY},
    start_server,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = env::args().into_iter();
    let mut port = "8080".to_string();
    let mut db_url = "127.0.0.1:6379".to_string();

    let mut internal_store = true;
    env::set_var(STORE_INTERNALLY, "true");
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
            env::set_var(STORE_INTERNALLY, "true");
        }
        false => {
            println!("connecting to DB {db_url}");
            env::set_var(STORE_INTERNALLY, "false");
            env::set_var(DB_URL, db_url);
        }
    }
    start_server(port).await
}
