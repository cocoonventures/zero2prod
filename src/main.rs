// use tokio::net::TcpListener;
// use std::env;
use std::net::TcpListener;

use sea_orm::error::DbErr;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
// use sea_orm::{entity::*, query::*};

#[allow(unused)]
use migration::{Migrator, MigratorTrait};

#[allow(unused)]
use serde::{Deserialize, Serialize};

use zero2prod::config::{get_config};
use zero2prod::startup::*;

async fn get_db(db_url: String) -> Result<DatabaseConnection, DbErr> {
    // let db_url = env::var("ZERO2PROD_DB_URL").expect("ENV[ZERO2PROD_DB] must be defined if database isn't defined in config.yml")

    let db: DatabaseConnection = Database::connect(db_url).await?;
    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read config.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let _db = get_db(config.database.connection_url())
        .await
        .expect("Problem getting db connection");
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
