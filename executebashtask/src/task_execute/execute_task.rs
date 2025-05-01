use sqlx::{Pool, Postgres};

pub async fn execute_task(id: i32, pg_pool: &Pool<Postgres>) {}
