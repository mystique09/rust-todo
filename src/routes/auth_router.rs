use axum::{extract::Extension, response::IntoResponse, Json};

use crate::{
    models::user_model::{Response, User},
    SharedStateDb,
};

pub struct AuthData {
    pub username: String,
    pub password: String,
}

pub async fn auth_rt(
    Json(auth_data): Json<AuthData>,
    Extension(state): Extension<SharedStateDb>,
) -> impl IntoResponse {
    let conn = state.conn.lock().unwrap();

    match User::check_user_by_uname(&conn, auth_data.username) {
        true => {
            // Authenticate user
            let user = User::get_user_by_username(&conn, auth_data.username).unwrap();
            if {
                Response::success("Logged In", Some(true))
            } else {
                Response::failure("Invalid password".to_string())
            }
        }
        false => Response::failure("User not founnd.".to_string()),
    }
}
