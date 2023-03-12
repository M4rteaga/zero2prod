use once_cell::sync::Lazy;
use sqlx::{Executor, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configurations, DatabaseSettings};
use zero2prod::startup::serve;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

//set up telemetry to only be initialize one time
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "debug".into();
    let subscriber_name = "test".into();

    //check for env variable TEST_LOG
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    //initialize telemetry
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut config = get_configurations().expect("Failed to get config");
    config.database.database_name = Uuid::new_v4().to_string();

    //the `Connection` trait MUST be in scope for us to invoke
    //`PgPool::connect` -it is not an inherent method of the struct!
    let connection_pool = configure_database(&config.database).await;

    let server = serve(listener, connection_pool.clone()).expect("Faild to bind address");
    let _ = tokio::spawn(server);

    let address = format!("http://127.0.0.1:{port}");
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    //create db
    let mut connection = PgPool::connect(&config.connection_string_withoud_db())
        .await
        .expect("Failed to connect to postgres");

    connection
        .execute(
            format!(
                r#"
                               CREATE DATABASE "{}";
                               "#,
                config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database");

    //Migrate
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let app = spawn_app().await;
    // We add `reqwest`
    // to perform HTTP requests agains our application
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Faild to execute recuest.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn suscribe_returns_400_for_invalid_form_data() {
    let app = spawn_app().await;
    // We add `reqwest`
    // to perform HTTP requests agains our application
    let client = reqwest::Client::new();
    //act
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlen-coded")
            .body(invalid_body)
            .send()
            .await
            .expect("Faild to execute recuest.");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not faild with 400 when the payload was {}",
            error_message
        )
    }
}
