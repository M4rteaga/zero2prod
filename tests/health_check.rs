use tokio::spawn;


#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app().await;
    // We add `reqwest`
    // to perform HTTP requests agains our application
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Faild to execute recuest.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = zero2prod::serve().expect("Faild to bind address");

    let _ = spawn(server);
}
