use r2d2::Pool;
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

    let redis_client =
        redis::Client::open("redis://127.0.0.1/").expect("Issue connecting to redis");
    let redis_pool = Pool::builder()
        .max_size(2)
        .build(redis_client)
        .expect("Issue creating connection pool for redis");

    loop {
        sleep(Duration::from_secs(5)).await;
        pull_from_db::db_interactor::pull_from_db(&pool, &redis_pool)
            .await
            .unwrap();
    }
}
