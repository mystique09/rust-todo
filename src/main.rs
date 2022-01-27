use axum::{response::Html, routing::get, Router};
use basic::routes::{
    todo_router::{all_todos_rt, delete_todo_rt, get_todo_rt, new_todo_rt, update_todo_rt},
    user_router::{all_user, user_by_id},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        // users routes
        .route("/", get(index_handler))
        .route("/users", get(all_user))
        .route("/user/:id", get(user_by_id))
        // todos routes
        .route("/todos", get(all_todos_rt).post(new_todo_rt))
        .route(
            "/todos/:id",
            get(get_todo_rt).delete(delete_todo_rt).put(update_todo_rt),
        );
    //.route("/todos/:id", delete(delete_todo_rt))
    //.rote("/todos/:id", put(update_todo_rt));

    println!("Listening on port {}", 3000);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html("Hello, World!")
}
