//! tests/health_check.rs
//!

// use tokio::net::TcpListener;
use actix_web::connect;
use config::Config;
use entities::prelude::*;
use entities::*;
use migration::{Migrator, MigratorTrait};
use sea_orm::entity::prelude::*;
use sea_orm::ConnectionTrait;
use sea_orm::{
    ConnectOptions, Database, DatabaseBackend, DatabaseConnection, DbBackend, ExecResult, Statement,
};
use std::net::TcpListener;
use std::time::Duration;
use uuid;
use zero2prod::config::*;
use zero2prod::startup::run;

pub struct TestApp {
    pub address: String,
    pub db_pool: ConnectOptions,
    pub test_db_name: String,
    pub config: Settings,
}

impl TestApp {
    async fn trash_test_db(&self) {
        if self
            .test_db_name
            .starts_with(&self.config.database.test_db_prefix)
        {
            let db = Database::connect(self.config.database.connection_url_nodb())
                .await
                .unwrap();
            let backend = get_backend(self.config.database.adapter.clone()).await;
            let destroy_db_str = format!("DROP DATABASE IF EXISTS `{}`;", self.test_db_name);
            db.execute(Statement::from_string(backend, destroy_db_str.to_owned()))
                .await
                .expect("Problem trashing the db");
        }
    }
}

async fn get_backend(backend_string: String) -> DatabaseBackend {
    let backend = match backend_string.as_str() {
        "postgres" => DbBackend::Postgres,
        "mysql" => DbBackend::MySql,
        "sqlite" => DbBackend::Sqlite,
        _ => DbBackend::Postgres,
    };
    backend
}

async fn configure_db(settings: DatabaseSettings) -> ConnectOptions {
    let backend = get_backend(settings.adapter.clone()).await;
    let db = Database::connect(settings.connection_url_nodb().clone())
        .await
        .expect("Problem connecting to url (config_db)");
    let create_db_str = format!("CREATE DATABASE \"{}\";", settings.db_name);
    let create_db_res: ExecResult = db
        .execute(Statement::from_string(backend, create_db_str.to_owned()))
        .await
        .expect("Problem creating test db");

    // have to reconnect to the actual database
    let db = Database::connect(settings.connection_url().clone())
        .await
        .expect("Problem connecting to url (config_db)");

    // Apply all pending migrations
    Migrator::up(&db, None)
        .await
        .expect("Problem migrating the test db");

    let mut db_pool = ConnectOptions::new(settings.connection_url());
    db_pool
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    db_pool
}

async fn spawn_app() -> TestApp {
    let mut config = get_config().expect("Failed to read config.");
    config.database.db_name = format!(
        "{}{}",
        config.database.test_db_prefix,
        Uuid::new_v4().to_string()
    );
    let db_pool = configure_db(config.database.clone()).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
        test_db_name: config.database.db_name.clone(),
        config,
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
    app.trash_test_db();
}

#[tokio::test]
async fn subcribe_returns_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let app_address = app.address.clone();
    // let config = get_config().expect("Failed to read config file.");
    // let connect_url: String = config.database.connection_url();

    let db: DatabaseConnection = Database::connect(app.db_pool.clone())
        .await
        .expect("Error connecting to test db pool");
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
    assert_eq!("ursula_le_guin@gmail.com", u.email);
    app.trash_test_db();
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
    app.trash_test_db();
}
