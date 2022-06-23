extern crate diesel;

use self::diesel::prelude::*;
use crate::schema;
use axum::{http::StatusCode, response::IntoResponse};
use diesel::result::Error as DbError;
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

#[derive(AsChangeset, Debug, Deserialize)]
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
    pub fn new_user(conn: &PgConnection, _create_user: CreateUser) -> Result<User, DbError> {
        let new_user = CreateUser {
            username: _create_user.username,
            password: _create_user.password,
            email: _create_user.email,
        };

        diesel::insert_into(userst::table)
            .values(&new_user)
            .get_result(conn)
    }

    pub fn get_user(conn: &PgConnection, _id: i32) -> Result<Option<User>, DbError> {
        let user = users.filter(id.eq(_id)).limit(1).load::<User>(conn);

        match user {
            Ok(mut u) => Ok(u.pop()),
            Err(why) => Err(why),
        }
    }

    pub fn get_users(conn: &PgConnection, page: i64) -> Result<Vec<User>, DbError> {
        users
            .filter(role.eq("Normal"))
            .limit(10)
            .offset(page)
            .load::<User>(conn)
    }

    pub fn get_user_by_username(
        conn: &PgConnection,
        _username: String,
    ) -> Result<Option<User>, DbError> {
        let user = users
            .filter(username.eq(_username))
            .limit(1)
            .load::<User>(conn);

        match user {
            Ok(mut u) => Ok(u.pop()),
            Err(why) => Err(why),
        }
    }

    pub fn update_user(conn: &PgConnection, _id: i32, _data: UpdateUser) -> Result<User, DbError> {
        diesel::update(userst::table)
            .filter(id.eq(_id))
            .set(_data)
            .get_result::<User>(conn)
    }

    pub fn delete_user(conn: &PgConnection, _id: i32) -> Result<Self, DbError> {
        diesel::delete(userst::table)
            .filter(id.eq(_id))
            .get_result::<User>(conn)
    }

    pub fn validate(self, pwd: String) -> bool {
        self.password.to_string() == pwd
    }
}

impl<'a, T: Serialize> IntoResponse for Response<'a, T> {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Self::Success { message, data } => (StatusCode::OK, (message.to_string(), data)),
            Self::Failure(error) => (StatusCode::BAD_REQUEST, (error, None)),
        };

        let parse_body = axum::Json(json!({ "message": body.0, "body": body.1 }));
        (status, parse_body).into_response()
    }
}

#[derive(Debug)]
pub enum Response<'a, T> {
    Success { message: &'a str, data: Option<T> },
    Failure(String),
}

impl<'a, T> Response<'a, T> {
    pub fn success(message: &'a str, data: Option<T>) -> Self {
        Self::Success { message, data }
    }

    pub fn failure(message: String) -> Self {
        Self::Failure(message)
    }
}
