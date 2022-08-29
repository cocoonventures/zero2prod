//! tests/health_check.rs
//!

fn spawn_app() -> std::io::Result<()> {
    todo!();
}
#[tokio::test]
async fn health_check_should_work() {
    spawn_app().await.expect("Failed to spawn app.");
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
