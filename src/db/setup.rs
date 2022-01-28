use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_conn() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    PgConnection::establish(&db_url).unwrap()
}
