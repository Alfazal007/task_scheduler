use chrono::{Duration, Utc};
use sqlx::{Pool, Postgres, query, query_as};

use crate::{models::tasks::IdAndType, pull_from_db::push_to_queue::push_to_redis_queue};

pub async fn pull_from_db(pg_pool: &Pool<Postgres>) -> Result<(), ()> {
    let cur_time = Utc::now();
    let future_time = cur_time + Duration::seconds(30);
    let epoch_time = future_time.timestamp();

    let tx_result = pg_pool.begin().await;
    if tx_result.is_err() {
        println!("Issue starting the transaction");
        return Err(());
    }

    let mut tx = tx_result.unwrap();
    let fetch_tasks_result = query_as::<_, IdAndType>(
        r#"
        SELECT id, "typeOfTask" FROM "Task"
        WHERE "scheduledAt" <= $1 AND "pickedAt" = 0
        ORDER BY "scheduledAt"
        LIMIT 2
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .bind(epoch_time)
    .fetch_all(&mut *tx)
    .await;

    if let Err(e) = fetch_tasks_result {
        println!("{:?}", e);
        let _ = tx.rollback().await;
        return Err(());
    };
    let tasks = fetch_tasks_result.unwrap();
    if tasks.is_empty() {
        let _ = tx.commit().await;
        return Ok(());
    }

    let queue_push_result = push_to_redis_queue(&tasks).await;
    if queue_push_result.is_err() {
        let _ = tx.rollback().await;
        return Err(());
    }

    let ids: Vec<i32> = tasks.iter().map(|item| item.id).collect();
    let picked_at_time = Utc::now().timestamp();
    let update_db_push_time_result = query!(
        r#"
        UPDATE "Task"
        set "pickedAt" = $1
        WHERE "id" = ANY($2)
        "#,
        picked_at_time,
        &ids[..] as &[i32]
    )
    .execute(&mut *tx)
    .await;

    if let Err(e) = update_db_push_time_result {
        println!("{:?}", e);
        let _ = tx.rollback().await;
        return Err(());
    }

    let _ = tx.commit().await;
    println!("Committed");
    Ok(())
}
