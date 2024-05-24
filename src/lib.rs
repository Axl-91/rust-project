pub mod tasks_controller;

use axum::{
    routing::{get, patch},
    Router,
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tokio::net::TcpListener;

pub async fn get_db_pool(database_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(16)
        .connect(database_url)
        .await
        .expect("Can't connect to database")
}

pub async fn bind_listener(server_address: String) -> TcpListener {
    TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener")
}

pub async fn set_app_routes(db_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to the Rust API Project" }))
        .route(
            "/tasks",
            get(tasks_controller::get_tasks).post(tasks_controller::create_task),
        )
        .route(
            "/task/:task_id",
            patch(tasks_controller::update_task).delete(tasks_controller::delete_task),
        )
        .with_state(db_pool)
}
