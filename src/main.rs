// use tokio::net::TcpListener;
// use std::env;

#[allow(unused_imports)]
use migration::{Migrator, MigratorTrait};
use sea_orm::error::DbErr;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::time::Duration;
use zero2prod::config::get_config;
use zero2prod::startup::*;

#[macro_use]
extern crate log;

#[allow(unused)]
async fn get_db(db_url: String) -> Result<DatabaseConnection, DbErr> {
    // let db_url = env::var("ZERO2PROD_DB_URL").expect("ENV[ZERO2PROD_DB] must be defined if database isn't defined in config.yml")

    let db: DatabaseConnection = Database::connect(db_url).await?;
    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let config = get_config().expect("Failed to read config.");
    let address = format!("127.0.0.1:{}", config.application_port);

    let mut db_pool = ConnectOptions::new(config.database.connection_url());
    db_pool
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    // let db = get_db(config.database.connection_url())
    //     .await
    //     .expect("Problem getting db connection");
    let listener = TcpListener::bind(address)?;
    run(listener, db_pool)?.await
}
