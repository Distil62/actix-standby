use crate::todo::todo_controller::{all, delete, get, insert, update};
use actix_web::{web, Scope};

pub fn get_todo_service() -> Scope {
    web::scope("/todo")
        .service(all)
        .service(get)
        .service(insert)
        .service(update)
        .service(delete)
}
