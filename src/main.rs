// use tokio::net::TcpListener;
use std::env;
use std::net::TcpListener;

use sea_orm::error::DbErr;
use sea_orm::Database;
use sea_orm::DatabaseConnection;

// use sea_orm::{entity::*, query::*};
// use serde::{Deserialize, Serialize};
#[allow(unused)]
use migration::{Migrator, MigratorTrait};
use zero2prod::startup::*;

async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("ZERO2PROD_DB_URL").expect("ZERO2PROD_DB must be defined");
    let db: DatabaseConnection = Database::connect(db_url).await?;
    Ok(db)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _db = get_db().await.expect("Problem getting db connection");

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
