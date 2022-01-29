use std::sync::{Arc, Mutex};

use diesel::PgConnection;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;

#[derive(Clone)]
pub struct SharedStateDb {
    pub conn: Arc<Mutex<PgConnection>>,
}
