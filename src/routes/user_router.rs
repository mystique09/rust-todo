use crate::{
    models::user_model::{CreateUser, Response, ResponseUser, UpdateUser, User},
    SharedStateDb,
};
use axum::{
    extract::{Extension, Path, Query},
    response::{IntoResponse, Json},
};
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DbError;
use serde::Deserialize;
use DbError::NotFound;

#[derive(Deserialize)]
pub struct UserOffsetQuery {
    page: Option<i64>,
}

pub async fn get_users_rt(
    offset: Query<UserOffsetQuery>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::get_users(&conn, offset.page.unwrap_or(0));

    match result {
        Ok(all_users) => {
            let response = all_users
                .into_iter()
                .map(|u| ResponseUser {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    role: u.role,
                })
                .collect::<Vec<ResponseUser>>();
            Response::success("All user", Some(response))
        }
        Err(why) => Response::failure(why.to_string()),
    }
}

pub async fn get_user_rt(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::get_user(&conn, id);

    match result {
        Ok(user) => match user {
            Some(data) => {
                let response = ResponseUser {
                    id: data.id,
                    username: data.username,
                    email: data.email,
                    role: data.role,
                };

                Response::success("User", Some(response))
            }
            None => Response::failure("User doesn't exist.".to_string()),
        },
        Err(why) => Response::failure(why.to_string()),
    }
}

pub async fn new_user_rt(
    Extension(state): Extension<SharedStateDb>,
    Json(create_user): Json<CreateUser>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let user = User::new_user(&conn, create_user);

    match user {
        Ok(new_user) => {
            let response = ResponseUser {
                id: new_user.id,
                username: new_user.username,
                email: new_user.email,
                role: new_user.role,
            };

            Response::success("New user", Some(response))
        }
        Err(DbError::DatabaseError(DatabaseErrorKind::UniqueViolation, violation)) => {
            Response::failure(violation.details().unwrap().to_string())
        }
        Err(_why) => Response::failure("Unable to create user.".to_string()),
    }
}

pub async fn update_user_rt(
    Path(id): Path<i32>,
    Json(update_user): Json<UpdateUser>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let result = User::update_user(&conn, id, update_user);

    match result {
        Ok(updated_user) => {
            let response = ResponseUser {
                id: updated_user.id,
                username: updated_user.username,
                email: updated_user.email,
                role: updated_user.role,
            };
            Response::success("Updated user", Some(response))
        }
        Err(DbError::DatabaseError(DatabaseErrorKind::__Unknown, err)) => {
            Response::failure(err.message().to_string())
        }
        Err(_why) => Response::failure(format!("Unable to update user: {}", _why)),
    }
}

pub async fn delete_user_rt(
    Path(id): Path<i32>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let del = User::delete_user(&conn, id);

    match del {
        Ok(deleted) => Response::success("User has been deleted.", Some(deleted.username)),
        Err(NotFound) => Response::failure("User doesn't exist.".to_string()),
        Err(why) => Response::failure(why.to_string()),
    }
}
