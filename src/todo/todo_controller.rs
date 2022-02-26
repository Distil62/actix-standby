use crate::database::DbPool;
use crate::diesel::RunQueryDsl;
use crate::error::{DbError, MyError};
use crate::todo::todo_model::{Todo, TodoPayload};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, Result};

#[get("/")]
pub async fn all(
    pool: web::Data<DbPool>,
    _req: HttpRequest,
) -> Result<web::Json<Vec<Todo>>, MyError> {
    use crate::schema::todos::dsl::*;
    let all_todos = web::block(move || -> Result<Vec<Todo>, DbError> {
        let connection = pool.get()?;
        let todo_responses = todos.get_results::<Todo>(&connection);

        Ok(todo_responses.unwrap())
    })
    .await
    .unwrap();

    Ok(web::Json(all_todos))
}

#[get("/get/:id")]
pub async fn get(info: web::Json<TodoPayload>) -> Result<String> {
    Ok(format!("label: {:?}, done: {}", info.label, info.done))
}

#[post("/")]
pub async fn insert(
    pool: web::Data<DbPool>,
    _info: web::Json<TodoPayload>,
) -> Result<web::Json<Todo>, MyError> {
    use crate::schema::todos::dsl::*;
    if _info.label == None {
        let insert_error = MyError {
            message: String::from("Label manquant veuillez renseigner un label"),
            status_code: StatusCode::BAD_REQUEST,
        };
        return Err(insert_error);
    }

    let inserted_todo_response = web::block(move || -> Result<Todo, DbError> {
        let connection = pool.get()?;

        let todo_to_insert: Todo = Todo {
            done: _info.done,
            id: uuid::Uuid::new_v4().to_string(),
            label: _info.label.as_ref().unwrap().to_string(),
        };

        diesel::insert_into(todos)
            .values(&todo_to_insert)
            .execute(&connection)?;

        Ok(todo_to_insert)
    })
    .await
    .unwrap();

    Ok(web::Json(Todo {
        id: inserted_todo_response.id,
        label: inserted_todo_response.label,
        done: inserted_todo_response.done,
    }))
}

#[get("/update/:id")]
pub async fn update(_req: HttpRequest) -> Result<String> {
    let json_todo = serde_json::to_string(&Todo {
        id: String::from("1"),
        label: String::from("update"),
        done: true,
    })
    .unwrap();
    Ok(json_todo)
}

#[get("/delete/:id")]
pub async fn delete(_req: HttpRequest) -> Result<String> {
    let json_todo = serde_json::to_string(&Todo {
        id: String::from("1"),
        label: String::from("update"),
        done: true,
    })
    .unwrap();
    Ok(json_todo)
}
