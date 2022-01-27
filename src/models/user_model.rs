use crate::db::setup::establish_conn;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create_user(_create_user: CreateUser) -> Self {
        unimplemented!();
    }
    pub fn find_by_id(_id: u32) -> Self {
        unimplemented!();
    }
    pub fn find_all() -> Vec<Self> {
        unimplemented!();
    }
    pub fn update_user(_id: u32, _data: CreateUser) -> Self {
        unimplemented!();
    }
    pub fn delete_user(_id: u32) -> Self {
        unimplemented!();
    }
}
