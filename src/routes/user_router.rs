use crate::models::user_model::User;
use axum::{
    extract::Path,
    response::{IntoResponse, Json},
};

pub async fn all_user() -> impl IntoResponse {
    let test_user: Vec<User> = vec![User {
        id: 1,
        username: "Benjie".to_string(),
        email: "testemail".to_string(),
        password: "testpass".to_string(),
        role: "Normal".to_string(),
    }];
    Json(test_user)
}

pub async fn user_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let user = User {
        id,
        username: "Test".to_string(),
        password: "test".to_string(),
        email: "test".to_string(),
        role: "Normal".to_string(),
    };
    Json(user)
}
