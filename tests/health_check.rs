use std::net::TcpListener;

use tokio::spawn;

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
    assert_eq!(200, response.status().as_u16())
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
    let server = zero2prod::serve(listener).expect("Faild to bind address");

    let _ = spawn(server);

    format!("http://127.0.0.1:{port}")
}
