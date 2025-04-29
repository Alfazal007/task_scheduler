use chrono::{Duration, Utc};
use sqlx::{Pool, Postgres, query_as};

use crate::models::tasks::Task;

pub async fn pull_from_db(pg_pool: &Pool<Postgres>) -> Result<(), ()> {
    let cur_time = Utc::now();
    let future_time = cur_time + Duration::minutes(5);
    let epoch_time = future_time.timestamp();

    let s = query_as::<_, Task>("select * from \"Task\"")
        .fetch_all(pg_pool)
        .await;
    if let Err(e) = s {
        println!("{:?}", e);
        return Err(());
    };
    let tasks = s.unwrap();
    for task in tasks.iter() {
        println!("{:?}", task);
    }
    Ok(())
}
