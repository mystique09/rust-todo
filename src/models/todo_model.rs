extern crate diesel;
use crate::schema;

use self::diesel::prelude::*;
use diesel::result::Error as DbError;
use diesel::{Insertable, Queryable};
use schema::todos_table as todost;
use serde::{Deserialize, Serialize};
use todost::dsl::*;

// A struct that represent the Todo model
// that can be inserted in the database.
#[derive(Debug, Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub author: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: bool,
}

// A struct that can be pass as a parameter
// of a route to bind the request body.
#[derive(Debug, Insertable, Deserialize)]
#[table_name = "todost"]
pub struct CreateTodo {
    pub author: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: bool,
}

// A struct that can be pass as a parameter
// of a route to bind the request body.
#[derive(Deserialize, AsChangeset)]
#[table_name = "todost"]
#[primary_key("id")]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: bool,
}

impl Todo {
    /* A method to query all todos from the database.
     * @return {Vec<Todo>}
     */
    pub fn all_todo(conn: &PgConnection) -> Result<Vec<Self>, DbError> {
        let result = todos_table
            .filter(completed.eq(false))
            .limit(10)
            .load::<Todo>(conn);

        result
    }
    /* A method to query a specific todo from the database.
     * @param {Todo::id} i32
     * @return {Todo}
     */
    pub fn get_todo(conn: &PgConnection, _id: i32) -> Result<Vec<Self>, DbError> {
        let result = todos_table.filter(id.eq(_id)).load::<Todo>(conn);
        result
    }
    /* A method to create a new todos
     * @param {CreateTodo}
     * @return {Todo}
     */
    pub fn create_todo(conn: &PgConnection, _create_todo: CreateTodo) -> Result<Self, DbError> {
        let new_todo = CreateTodo {
            author: _create_todo.author,
            title: _create_todo.title,
            description: _create_todo.description,
            completed: false,
        };

        diesel::insert_into(todost::table)
            .values(&new_todo)
            .get_result(conn)
    }
    // A methor to update a new todo.
    // @param {UpdateTodo}
    // @return {Todo}
    pub fn update_todo(
        conn: &PgConnection,
        _id: i32,
        _update_todo: UpdateTodo,
    ) -> Result<Self, DbError> {
        diesel::update(todost::table)
            .filter(id.eq(_id))
            .set(_update_todo)
            .get_result::<Todo>(conn)
    }
}
