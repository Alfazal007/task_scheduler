use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};
use tokio::time::{Duration, sleep};

pub mod models;
pub mod pull_from_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("Issue connecting to the database");

    loop {
        sleep(Duration::from_secs(5)).await;
        pull_from_db::db_interactor::pull_from_db(&pool)
            .await
            .unwrap();
    }
}
