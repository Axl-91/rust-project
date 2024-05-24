use rust_project::{bind_listener, get_db_pool, set_app_routes};

fn get_env_var() -> Result<(String, String), dotenvy::Error> {
    dotenvy::dotenv()?;

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env file");

    Ok((server_address, database_url))
}

#[tokio::main]
async fn main() {
    let (server_address, database_url) = get_env_var().unwrap();

    let db_pool = get_db_pool(&database_url).await;

    let listener = bind_listener(server_address).await;

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = set_app_routes(db_pool).await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application")
}
