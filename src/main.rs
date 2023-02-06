use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::serve;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //set db connection
    let config = get_configurations().expect("Faild to get config");
    let db_connection = PgConnection::connect(&config.database.connection_string())
        .await
        .expect("Faild to connect to the database");
    //panic if we cant read configuration
    let config = get_configurations().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener: TcpListener = TcpListener::bind(address).expect("Failed to bind port");
    serve(listener, db_connection)?.await
}
