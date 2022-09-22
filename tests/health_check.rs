//! tests/health_check.rs
//!

// use tokio::net::TcpListener;
use actix_web::connect;
use sea_orm::entity::prelude::*;
use sea_orm::Database;
use sea_orm::DatabaseConnection;

// use entities::subscription::Relation::User;
use entities::prelude::*;
use entities::*; //subscription::*;
                 // use entities::*;

use std::net::TcpListener;
use zero2prod::config::get_config;
use zero2prod::startup::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_should_work() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subcribe_returns_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let config = get_config().expect("Failed to read config file.");
    let connect_url = config.database.connection_url();

    let db: DatabaseConnection = Database::connect(connect_url).await.unwrap();
    let client = reqwest::Client::new();

    // Act
    let body = "name=Le%20Guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let s = Subscription::find()
        .one(&db)
        .await
        .expect("Failed to fetch subscription");
    let s: subscription::Model = s.expect("Failed to fetch a saved subscription"); //unwrap();
    let u = s
        .find_related(User)
        .one(&db)
        .await
        .expect("Failed to fetch a user in connection with a subscription");
    let u: user::Model = u.unwrap();

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("Le Guin", u.name);
    assert_eq!("ursula_le_guin@gmail.com", u.email)
}

#[tokio::test]
async fn subscribe_return_400_for_missing_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Le%20Guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_msg
        );
    }
}
