use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use tokio::spawn;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::serve;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app().await;
    // We add `reqwest`
    // to perform HTTP requests agains our application
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Faild to execute recuest.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn suscribe_returns_200_for_valid_form_data() {
    let app_address = spawn_app().await;
    let config = get_configurations().expect("Failed to get config");
    let connection_string = config.database.connection_string();

    //the `Connection` trait MUST be in scope for us to invoke
    //`PgConnection::connect` -it is not an inherent method of the struct!
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Faild to connect to postgres");

    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn suscribe_returns_400_for_invalid_form_data() {
    let address = spawn_app().await;
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
            .post(&format!("{}/subscriptions", &address))
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

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = serve(listener).expect("Faild to bind address");

    let _ = spawn(server);

    format!("http://127.0.0.1:{port}")
}
