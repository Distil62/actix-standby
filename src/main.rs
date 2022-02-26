// https://blog.guillaume-gomez.fr/Rust

#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use todo::get_todo_service;

mod database;
mod error;
mod schema;
mod todo;

#[get("/ping")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool = database::database_connection();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(logger)
            .service(hello)
            .service(get_todo_service())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
