use diesel::PgConnection;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;
use tower_cookies::Key;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;

#[derive(Clone)]
pub struct SharedStateDb {
    pub conn: Arc<Mutex<PgConnection>>,
    pub cookie_secret: Arc<Mutex<OnceCell<Key>>>,
}

/*
Auth guard, checks if user is logged in
*/
pub struct Auth;
