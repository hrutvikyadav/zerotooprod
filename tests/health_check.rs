
//! tests/health_check.rs
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let api_base_url: String = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let health_check_endpoint = format!("{}/health_check", api_base_url);
    // Act
    let response = client
        .get(health_check_endpoint)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success()); assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    const RANDOM_AV_SOCK: &str = "127.0.0.1:0";
    let listener = TcpListener::bind(RANDOM_AV_SOCK)
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zerotooprod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let api_base_url: String = spawn_app();
    let client = reqwest::Client::new();
    let subscribe_endpoint = format!("{}/subscribe", api_base_url);
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(subscribe_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    // Arrange
    let api_base_url: String = spawn_app();
    let client = reqwest::Client::new();
    let subscribe_endpoint = format!("{}/subscribe", api_base_url);
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, reason) in test_cases {
        // Act
        let response = client
            .post(&subscribe_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(400, response.status().as_u16(),
            "expected /subscribe to fail with 400 status because form was {}.",
            reason
        );
    }
}
