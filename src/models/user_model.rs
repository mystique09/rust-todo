extern crate diesel;
use self::diesel::prelude::*;

use crate::db::setup::establish_conn;
use crate::schema;
use axum::{http::StatusCode, response::IntoResponse};
use schema::users as userst;
use serde::{Deserialize, Serialize};
use serde_json::json;
use userst::dsl::*;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "userst"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Identifiable, AsChangeset, Debug, Deserialize)]
#[table_name = "userst"]
#[primary_key("id")]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl User {
    pub fn new_user(_create_user: CreateUser) -> Result<Self, CreationError<'static>> {
        let conn = establish_conn();
        let new_user = CreateUser {
            username: _create_user.username,
            password: _create_user.password,
            email: _create_user.email,
        };

        let user = diesel::insert_into(userst::table)
            .values(&new_user)
            .get_result(&conn);

        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(CreationError::DuplicateKey(
                "Email or username is already in used.",
            )),
        }
    }

    pub fn get_user(_id: i32) -> Vec<Self> {
        let conn = establish_conn();

        let result = users.filter(id.eq(_id)).load::<User>(&conn).unwrap();
        result
    }
    pub fn get_users() -> Vec<ResponseUser> {
        let conn = establish_conn();

        let results = users
            .filter(role.eq("Normal"))
            .limit(10)
            .load::<User>(&conn)
            .unwrap();
        results
            .into_iter()
            .map(|user| {
                let new_res_user = ResponseUser {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    role: user.role,
                };
                new_res_user
            })
            .collect()
    }
    pub fn update_user(_id: i32, _data: UpdateUser) -> Result<ResponseUser, String> {
        let conn = establish_conn();
        // let target = users.filter(id.eq(_id)).load::<User>(&conn).unwrap().get(0);

        let updated = diesel::update(userst::table)
            .filter(id.eq(_id))
            .set(_data)
            .get_result::<User>(&conn);

        match updated {
            Ok(updated) => {
                let response = ResponseUser {
                    id: _id,
                    username: updated.username,
                    email: updated.email,
                    role: updated.role,
                };
                Ok(response)
            }
            Err(err) => Err(format!("Unable to update user: {}", err)),
        }

        /*"let response = ResponseUser {
            id: _id,
            username: updated.username,
            email: updated.email,
            role: updated.role,
        };*/
    }

    pub fn delete_user(_id: i32) -> Self {
        unimplemented!();
    }
}

impl<'a> IntoResponse for CreationError<'a> {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            CreationError::EmailIsUsed => (StatusCode::BAD_REQUEST, "Email is already in used."),
            CreationError::UserAlreadyExist => (StatusCode::BAD_REQUEST, "User already exist."),
            CreationError::SomethingWentWrong => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.")
            }
            CreationError::DuplicateKey(err) => (StatusCode::BAD_REQUEST, err),
        };

        let error_m = axum::Json(json!({ "err": body }));
        (status, error_m).into_response()
    }
}

// An enum for to respond for errors when creatint new users.
#[derive(Debug)]
pub enum CreationError<'a> {
    // UserAlreadyExist variant
    UserAlreadyExist,
    // If emaail is en use
    EmailIsUsed,
    //Something went wrong
    SomethingWentWrong,
    // Duplicate
    DuplicateKey(&'a str),
}

impl<'a> IntoResponse for Response<'a> {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Response::Success { message, data } => (StatusCode::OK, (message.to_string(), data)),
            Response::Failure(error) => (StatusCode::BAD_REQUEST, (error.to_string(), None)),
        };

        let parse_body = axum::Json(json!({ "body": body }));
        (status, parse_body).into_response()
    }
}

#[derive(Debug, Serialize)]
pub enum Response<'a> {
    Success {
        message: &'a str,
        data: Option<ResponseUser>,
    },
    Failure(String),
}
