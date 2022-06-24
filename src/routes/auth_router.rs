use axum::{extract::Extension, response::IntoResponse, Json};
use tower_cookies::{Cookie, Cookies};

use crate::{
    models::user_model::{Response, User},
    SharedStateDb,
};

pub struct AuthData {
    pub username: String,
    pub password: String,
}

pub async fn auth_index() -> impl IntoResponse {
    Response::success("You are in login endpoint", Some(true))
}

pub async fn auth_rt(
    cookies: Cookies,
    Json(auth_data): Json<AuthData>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();
    let key = state.cookie_secret.lock().unwrap();

    let username = auth_data.username;
    let session_cookie = cookies.signed(key.get().unwrap());

    match User::get_user_by_username(&conn, username) {
        Ok(user) => match user {
            Some(data) => {
                if data.validate(auth_data.password) {
                    session_cookie.add(Cookie::new("session_cookie", "testcookie"));
                    Response::success("Logged in.", Some(true))
                } else {
                    Response::failure("Incorrect password.".to_string())
                }
            }
            None => Response::failure("User doesn't exist.".to_string()),
        },
        Err(why) => Response::failure(why.to_string()),
    }
}
