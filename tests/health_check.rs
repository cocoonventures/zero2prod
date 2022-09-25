//! tests/health_check.rs
//!

// use tokio::net::TcpListener;
use actix_web::connect;
use sea_orm::entity::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

// use entities::subscription::Relation::User;
use entities::prelude::*;
use entities::*; //subscription::*;
                 // use entities::*;

use std::net::TcpListener;
use zero2prod::config::get_config;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: ConnectOptions,
}

async fn spawn_app() -> TestApp {
    let config = get_config().expect("Failed to read config.");
    let mut db_pool = ConnectOptions::new(config.database.connection_url());
    db_pool
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: db_pool,
    }
}

#[tokio::test]
async fn health_check_should_work() {
    let app = spawn_app().await;
    let address = app.address;
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
    let app = spawn_app().await;
    let app_address = app.address;
    let config = get_config().expect("Failed to read config file.");
    // let connect_url: String = config.database.connection_url();

    let db: DatabaseConnection = Database::connect(app.db_pool).await.unwrap();
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
        .expect("Failed with a DbErr")
        .expect("Failed to fetch a saved subscription");
    // let s: subscription::Model = s.expect("Failed to fetch a saved subscription"); //unwrap();
    let u = s
        .find_related(User)
        .one(&db)
        .await
        .expect("Finding related user failed with a DbErr")
        .expect("Failed to fetch a user in connection with a subscription");

    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!("Le Guin", u.name);
    assert_eq!("ursula_le_guin@gmail.com", u.email)
}

#[tokio::test]
async fn subscribe_return_400_for_missing_data() {
    let app = spawn_app().await;
    let app_address = app.address;
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
