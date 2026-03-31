
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
