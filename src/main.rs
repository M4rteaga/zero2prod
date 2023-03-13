use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::serve;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    //set db connection
    let config = get_configurations().expect("Faild to get config");
    let db_connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Faild to connect to the database");
    //panic if we cant read configuration
    let config = get_configurations().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind port");
    serve(listener, db_connection_pool)?.await
}
