// use tokio::net::TcpListener;
use std::env;
use std::net::TcpListener;

use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
// use sea_orm::{entity::*, query::*};
// use serde::{Deserialize, Serialize};

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("ZERO2PROD_DB_URL")?;
    let db: DatabaseConnection = Database::connect(db_url).await?;


    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
