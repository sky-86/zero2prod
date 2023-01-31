use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
//use zero2prod::telemetry::get_otel_subscriber;
use zero2prod::{
    config::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    //let subscriber = get_otel_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config file");

    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.app_port))
        .expect("Failed to bind to port");

    run(listener, connection_pool)?.await
}
