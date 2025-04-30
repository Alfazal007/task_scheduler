use chrono::{Duration, Utc};
use sqlx::{Pool, Postgres, query};

pub async fn pull_from_db(pg_pool: &Pool<Postgres>) -> Result<(), ()> {
    let cur_time = Utc::now();
    let future_time = cur_time + Duration::seconds(30);
    let epoch_time = future_time.timestamp();

    let mut tx_result = pg_pool.begin().await;
    if tx_result.is_err() {
        println!("Issue starting the transaction");
        return Err(());
    }

    let mut tx = tx_result.unwrap();
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = tx_result.unwrap();
    let fetch_tasks_result= query!("select * from \"Task\" where \"scheduledAt\" <= $1 and \"pickedAt\"=0 order by \"scheduledAt\" limit 2  for update skip locked", epoch_time)
        .fetch_all(&mut tx)
        .await;

    if let Err(e) = fetch_tasks_result {
        println!("{:?}", e);
        return Err(());
    };
    let tasks = fetch_tasks_result.unwrap();
    for task in tasks.iter() {
        println!("{:?}", task);
    }
    tx.commit();

    Ok(())
}
