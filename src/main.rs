use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{config::get_config, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config file");

    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(&format!("127.0.0.1:{}", config.app_port))
        .expect("Failed to bind to port");

    run(listener, connection_pool)?.await
}
