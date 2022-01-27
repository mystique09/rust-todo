use axum::{
    extract::{Json, Path},
    response::IntoResponse,
};

use crate::models::todo_model::{CreateTodo, Todo, UpdateTodo};

pub async fn all_todos_rt() -> impl IntoResponse {
    let todos = Todo::all_todo();
    Json(todos)
}

pub async fn get_todo_rt(Path(id): Path<i32>) -> impl IntoResponse {
    let todo = Todo::get_todo(id);
    Json(todo)
}

pub async fn new_todo_rt(Json(_create_todo): Json<CreateTodo>) -> impl IntoResponse {
    let new_todo = Todo::create_todo(_create_todo);
    Json(new_todo)
}

pub async fn update_todo_rt(
    Path(_id): Path<i32>,
    Json(_update_todo): Json<UpdateTodo>,
) -> impl IntoResponse {
    Json("[UPDATE] /todos/:id")
}

pub async fn delete_todo_rt(Path(_id): Path<i32>) -> impl IntoResponse {
    Json("[DELETE] /todos/:id")
}
