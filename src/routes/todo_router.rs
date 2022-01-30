use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
};
use diesel::result::Error as DbError;

use crate::{
    models::todo_model::{CreateTodo, Todo, UpdateTodo},
    SharedStateDb,
};

use crate::models::user_model::Response;

pub async fn all_todos_rt(Extension(state): Extension<SharedStateDb>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let todos = Todo::all_todo(&conn);

    match todos {
        Ok(todos) => Response::success("All todos", Some(todos)),
        Err(_why) => Response::failure(_why.to_string()),
    }
}

pub async fn get_todo_rt(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let todo = Todo::get_todo(&conn, id);

    match todo {
        Ok(result) => Response::success("Todo", Some(result)),
        Err(DbError::NotFound) => Response::failure("Todo not found!".to_string()),
        Err(_why) => Response::failure(_why.to_string()),
    }
}

pub async fn new_todo_rt(
    Json(create_todo): Json<CreateTodo>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let new_todo = Todo::create_todo(&conn, create_todo);

    match new_todo {
        Ok(todo) => Response::success("New todo", Some(todo)),
        Err(_why) => Response::failure(_why.to_string()),
    }
}

pub async fn update_todo_rt(
    Path(id): Path<i32>,
    Json(update_todo): Json<UpdateTodo>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let todo = Todo::update_todo(&conn, id, update_todo);

    match todo {
        Ok(updated_todo) => Response::success("Updated todo", Some(updated_todo)),
        Err(DbError::NotFound) => Response::failure(format!(
            "Unable to udpated todo, todo with id {} not found.",
            id
        )),
        Err(DbError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, error)) => {
            Response::failure(error.message().to_string())
        }
        Err(_why) => Response::failure("Something went wrong.".to_string()),
    }
}

pub async fn delete_todo_rt(Path(_id): Path<i32>) -> impl IntoResponse {
    Json("[DELETE] /todos/:id")
}
