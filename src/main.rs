use axum::{response::Html, routing::get, AddExtensionLayer, Router};
use basic::SharedStateDb;
use basic::{
    db::setup::establish_conn,
    routes::{
        todo_router::{all_todos_rt, delete_todo_rt, get_todo_rt, new_todo_rt, update_todo_rt},
        user_router::{delete_user_rt, get_user_rt, get_users_rt, new_user_rt, update_user_rt},
    },
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() {
    let conn = Arc::new(Mutex::new(establish_conn()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        // main route
        .route("/", get(index_handler))
        // user routes
        .route("/users", get(get_users_rt).post(new_user_rt))
        .route(
            "/users/:id",
            get(get_user_rt).delete(delete_user_rt).put(update_user_rt),
        )
        // todos routes
        .route("/todos", get(all_todos_rt).post(new_todo_rt))
        .route(
            "/todos/:id",
            get(get_todo_rt).delete(delete_todo_rt).put(update_todo_rt),
        )
        .layer(AddExtensionLayer::new(SharedStateDb { conn }));

    println!("Listening on port {}", 3000);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello, World!")
}
