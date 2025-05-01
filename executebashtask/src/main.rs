use core::panic;
use redis::Commands;
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};
use task_execute::start_task::start_task;

pub mod models;
pub mod task_execute;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("Issue connecting to the database");
    let mut redis_client = redis::Client::open("redis://127.0.0.1/")
        .expect("Issue connecting to redis")
        .get_connection()
        .expect("Could not get redis connection");

    loop {
        let redis_result: redis::RedisResult<(String, i32)> = redis_client.brpop("BASH", 0.0);
        if let Err(e) = redis_result {
            panic!("Issue pulling redis data {:?}", e);
        }
        let id = redis_result.unwrap().1;
        let res = start_task(id, &pool).await;
        if let Err(e) = res {
            println!("{:?}", e);
        }
    }
}
