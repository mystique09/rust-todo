use crate::{
    models::user_model::{CreateUser, CreationError, Response, UpdateUser, User},
    SharedStateDb,
};
use axum::{
    extract::{Extension, Path},
    response::{IntoResponse, Json},
};
pub async fn get_users_rt(Extension(state): Extension<SharedStateDb>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::get_users(&conn);
    Json(result)
}

pub async fn get_user_rt(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::get_user(&conn, id);
    Json(result)
}

pub async fn new_user_rt(
    Extension(state): Extension<SharedStateDb>,
    Json(create_user): Json<CreateUser>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let user = User::new_user(&conn, create_user);
    match user {
        Ok(_user) => Json("New user added."),
        Err(CreationError::DuplicateKey(err)) => Json(err),
        Err(_) => Json("Something went wrong"),
    }
}

pub async fn update_user_rt(
    Path(_id): Path<i32>,
    Json(_update_user): Json<UpdateUser>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::update_user(&conn, _id, _update_user);

    match result {
        Ok(response) => Response::Success {
            message: "User updated",
            data: Some(response),
        },
        Err(err) => Response::Failure(err),
    }
}

pub async fn delete_user_rt(
    Path(_id): Path<i32>,
    Extension(_state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    //todo!();
    unimplemented!();
}
