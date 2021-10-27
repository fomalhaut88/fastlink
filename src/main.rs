extern crate dotenv;

use std::env;
use std::sync::Mutex;

use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;

use fastlink::appstate::AppState;
use fastlink::db::DBConnector;
use fastlink::views::{version, get, add};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Environment
    dotenv().ok();

    let host: &str = &env::var("FASTLINK_HOST")
        .unwrap_or("127.0.0.1".to_string())[..];
    let port: u16 = env::var("FASTLINK_PORT")
        .unwrap_or("8080".to_string()).parse::<u16>().unwrap();
    let url_max_length: usize = env::var("URL_MAX_LENGTH")
        .unwrap().parse::<usize>().unwrap();
    let db_data_path: &str = &env::var("DB_DATA_PATH")
        .unwrap_or("db/fastlink.data".to_string())[..];
    let db_state_path: &str = &env::var("DB_STATE_PATH")
        .unwrap_or("db/fastlink.state".to_string())[..];
    let db_order: usize = env::var("DB_ORDER")
        .unwrap().parse::<usize>().unwrap();
    let db_prime: u64 = env::var("DB_PRIME")
        .unwrap().parse::<u64>().unwrap();
    let db_generator: u64 = env::var("DB_GENERATOR")
        .unwrap().parse::<u64>().unwrap();

    // Define appstate
    let appstate = web::Data::new(AppState {
        db: Mutex::new(DBConnector::new(
            &db_data_path, &db_state_path,
            db_prime, db_generator, 0, url_max_length
        )),
        order: db_order,
        prime: db_prime,
        block_size: url_max_length,
    });

    // Initialize database
    {
        let mut db = appstate.db.lock().unwrap();
        db.ensure(db_order);
    };

    // Create and run webserver
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(appstate.clone())
            .service(version)
            .service(get)
            .service(add)
    })
        .bind((host, port))?
        .run()
        .await
}
