use std::net::SocketAddr;

mod http;
mod snowflake;

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=migrations");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Environment variables
    let port = std::env::var("PORT")
        .unwrap_or("5005".to_string())
        .parse::<u16>()
        .unwrap();
    let database_username = std::env::var("DATABASE_USERNAME").unwrap_or("postgres".to_string());
    let database_password = std::env::var("DATABASE_PASSWORD").unwrap();
    let database_host = std::env::var("DATABASE_HOST").unwrap_or("localhost".to_string());
    let database_port = std::env::var("DATABASE_PORT")
        .unwrap_or("5432".to_string())
        .parse::<u16>()
        .unwrap();
    let database_name = std::env::var("DATABASE_NAME").unwrap_or("postgres".to_string());

    let openssl_encryption_key = std::env::var("OPENSSL_ENCRYPTION_KEY").unwrap();
    let session_secret = std::env::var("SESSION_SECRET").unwrap();

    // Establish database connection
    tracing::info!(
        "Attempting to establish database connection on {database_host}:{database_port}"
    );
    let connection_url = format!("postgres://{database_username}:{database_password}@{database_host}:{database_port}/{database_name}");
    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(50)
        .connect(&connection_url)
        .await
        .unwrap();
    tracing::info!("Database connection established, running pending migrations");
    sqlx::migrate!().run(&database).await.unwrap();

    // Serve application
    let (application, listener) =
        http::get_app(port, database, openssl_encryption_key, session_secret).await;
    tracing::info!("Starting server on port {port}");
    axum::serve(
        listener,
        application.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
