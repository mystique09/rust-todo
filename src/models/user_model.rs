extern crate diesel;
use self::diesel::prelude::*;
use crate::{db::setup::establish_conn, schema};
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
    pub fn new_user(conn: &PgConnection, _create_user: CreateUser) -> Result<User, DbError> {
        let new_user = CreateUser {
            username: _create_user.username,
            password: _create_user.password,
            email: _create_user.email,
        };

        let user = diesel::insert_into(userst::table)
            .values(&new_user)
            .get_result(conn);

        user
    }

    pub fn get_user(conn: &PgConnection, _id: i32) -> Result<Vec<Self>, DbError> {
        let result = users.filter(id.eq(_id)).load::<User>(conn);
        result
    }
    pub fn get_users(conn: &PgConnection) -> Result<Vec<User>, DbError> {
        let results = users.filter(role.eq("Normal")).limit(10).load::<User>(conn);
        results
    }
    pub fn update_user(conn: &PgConnection, _id: i32, _data: UpdateUser) -> Result<User, DbError> {
        let has_user = User::check_user_by_id(_id);

        if has_user == false {
            return Err(DbError::NotFound);
        }

        let updated = diesel::update(userst::table)
            .filter(id.eq(_id))
            .set(_data)
            .get_result::<User>(conn);

        updated
    }

    pub fn delete_user(_conn: &PgConnection, _id: i32) -> Self {
        unimplemented!();
    }

    pub fn check_user_by_id(uid: i32) -> bool {
        let conn = establish_conn();
        let user = users.filter(id.eq(uid)).load::<User>(&conn);

        match user {
            Ok(user) => match user.get(0) {
                Some(_existing_user) => true,
                None => false,
            },
            Err(_) => false,
        }
    }

    pub fn check_user_by_uname(uname: String) -> bool {
        let conn = establish_conn();
        let user = users.filter(username.eq(uname)).load::<User>(&conn);

        match user {
            Ok(user) => match user.get(0) {
                Some(_existing_user) => true,
                None => false,
            },
            Err(_) => false,
        }
    }
}

impl<'a, T: Serialize> IntoResponse for Response<'a, T> {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Self::Success { message, data } => (StatusCode::OK, (message.to_string(), data)),
            Self::Failure(error) => (StatusCode::BAD_REQUEST, (error.to_string(), None)),
        };

        let parse_body = axum::Json(json!({ "body": body }));
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
