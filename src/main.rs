use axum::{
    routing::{get, patch},
    Router,
};

use sqlx::postgres::PgPoolOptions;

use tokio::net::TcpListener;

mod tasks_controller;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Can't connect to database");

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async {"Welcome to the Rust API Project"}))
        .route("/tasks", get(tasks_controller::get_tasks).post(tasks_controller::create_task))
        .route("/task/:task_id", patch(tasks_controller::update_task).delete(tasks_controller::delete_task))
        .with_state(db_pool);

    axum::serve(listener, app)
        .await
        .expect("Error serving application")
}
