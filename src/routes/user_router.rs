use crate::{
    models::user_model::{CreateUser, Response, UpdateUser, User},
    SharedStateDb,
};
use axum::{
    extract::{Extension, Path},
    response::{IntoResponse, Json},
};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DbError;

pub async fn get_users_rt(Extension(state): Extension<SharedStateDb>) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::get_users(&conn);

    match result {
        Ok(all_users) => Response::success("All user", Some(all_users)),
        Err(_) => Response::failure("Database Error".to_string()),
    }
}

pub async fn get_user_rt(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::get_user(&conn, id);

    match result {
        Ok(ok_user) => Response::success("User", Some(ok_user)),
        Err(DbError::NotFound) => Response::failure("User not found".to_string()),
        Err(_why) => Response::failure(_why.to_string()),
    }
}

pub async fn new_user_rt(
    Extension(state): Extension<SharedStateDb>,
    Json(create_user): Json<CreateUser>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let user = User::new_user(&conn, create_user);

    match user {
        Ok(new_user) => Response::success("New user", Some(new_user)),
        Err(DbError::DatabaseError(DatabaseErrorKind::UniqueViolation, violation)) => {
            Response::failure(violation.details().unwrap().to_string())
        }
        Err(_why) => Response::failure("Unable to create user".to_string()),
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
        Ok(updated_user) => Response::success("Updated user", Some(updated_user)),
        Err(DbError::DatabaseError(DatabaseErrorKind::__Unknown, err)) => {
            Response::failure(err.message().to_string())
        }
        Err(_why) => Response::failure(format!("Unable to update user: {}", _why)),
    }
}

pub async fn delete_user_rt(
    Path(_id): Path<i32>,
    Extension(_state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    //todo!();
    unimplemented!();
}
