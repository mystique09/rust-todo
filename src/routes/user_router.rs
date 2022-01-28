use crate::models::user_model::{CreateUser, CreationError, UpdateUser, User};
use axum::{
    extract::Path,
    response::{IntoResponse, Json},
};
pub async fn get_users_rt() -> impl IntoResponse {
    let result = User::get_users();
    Json(result)
}

pub async fn get_user_rt(Path(id): Path<i32>) -> impl IntoResponse {
    let result = User::get_user(id);
    Json(result)
}

pub async fn new_user_rt(Json(create_user): Json<CreateUser>) -> impl IntoResponse {
    let user = User::new_user(create_user);
    match user {
        Ok(_user) => Json("New user added."),
        Err(CreationError::DuplicateKey(err)) => Json(err),
        Err(_) => Json("Something went wrong"),
    }
}

pub async fn update_user_rt(
    Path(_id): Path<i32>,
    Json(_update_user): Json<UpdateUser>,
) -> impl IntoResponse {
    //todo!();
    Json("unimplemented!()")
}

pub async fn delete_user_rt(Path(_id): Path<i32>) -> impl IntoResponse {
    //todo!();
    Json("unimplemented!()")
}
