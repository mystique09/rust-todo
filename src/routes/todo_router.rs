use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
};

use crate::{
    models::todo_model::{CreateTodo, Todo, UpdateTodo},
    SharedStateDb,
};

pub async fn all_todos_rt(Extension(state): Extension<SharedStateDb>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let todos = Todo::all_todo(&conn);
    Json(todos)
}

pub async fn get_todo_rt(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let todo = Todo::get_todo(&conn, id);
    Json(todo)
}

pub async fn new_todo_rt(
    Json(_create_todo): Json<CreateTodo>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let new_todo = Todo::create_todo(&conn, _create_todo);
    Json(new_todo)
}

pub async fn update_todo_rt(
    Path(_id): Path<i32>,
    Json(_update_todo): Json<UpdateTodo>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let _todos = Todo::update_todo(&conn, _update_todo);
    Json("unimplemented!()")
}

pub async fn delete_todo_rt(Path(_id): Path<i32>) -> impl IntoResponse {
    Json("[DELETE] /todos/:id")
}
