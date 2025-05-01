use chrono::Utc;
use sqlx::{Pool, Postgres, query, query_as};

use crate::models::tasks::Task;

use super::execute_task::execute_task;

pub async fn start_task(id: i32, pg_pool: &Pool<Postgres>) -> Result<(), String> {
    let tx_result = pg_pool.begin().await;
    if tx_result.is_err() {
        return Err("Issue initiating database transaction".to_string());
    }

    let mut tx = tx_result.unwrap();
    let fetch_task_result = query_as::<_, Task>(
        r#"
        SELECT * FROM "Task"
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await;
    if fetch_task_result.is_err() {
        let _ = tx.rollback().await;
        return Err(format!(
            "Issue fetching the task with the id = {:?} from the database",
            id,
        ));
    }

    if fetch_task_result.as_ref().unwrap().is_none() {
        let _ = tx.rollback().await;
        return Err(format!(
            "The task with the id = {:?} could not be found in the database",
            id,
        ));
    }

    let started_at_time = Utc::now().timestamp();
    let update_db_push_time_result = query!(
        r#"
        UPDATE "Task"
        set "startedAt" = $1
        WHERE "id" = $2
        "#,
        started_at_time,
        id
    )
    .execute(&mut *tx)
    .await;
    if let Err(e) = update_db_push_time_result {
        let _ = tx.rollback().await;
        return Err(format!(
            "Failed task id = {:?}, with error = {:?}",
            id,
            e.to_string()
        ));
    }

    let execute_task_result = execute_task(fetch_task_result.unwrap().unwrap().command).await;
    if execute_task_result.is_ok() {
        let finish_time = Utc::now().timestamp();
        let update_db_push_time_result = query!(
            r#"
        UPDATE "Task"
        set "completedAt" = $1
        WHERE "id" = $2
        "#,
            finish_time,
            id
        )
        .execute(&mut *tx)
        .await;
        if let Err(e) = update_db_push_time_result {
            let _ = tx.rollback().await;
            return Err(format!(
                "Failed task id = {:?}, with error = {:?}",
                id,
                e.to_string()
            ));
        }
        let _ = tx.commit().await;
        return Ok(());
    }

    let failed_time = Utc::now().timestamp();
    let update_db_push_time_result = query!(
        r#"
        UPDATE "Task"
        set "failedAt" = $1
        WHERE "id" = $2
        "#,
        failed_time,
        id
    )
    .execute(&mut *tx)
    .await;
    if let Err(e) = update_db_push_time_result {
        let _ = tx.rollback().await;
        return Err(format!(
            "Failed task id = {:?}, with error = {:?}",
            id,
            e.to_string()
        ));
    }
    let _ = tx.commit().await;
    Ok(())
}
