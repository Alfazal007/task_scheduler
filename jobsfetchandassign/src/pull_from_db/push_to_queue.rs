use r2d2::Pool;
use redis::{Client, Commands};

use crate::models::tasks::IdAndType;

pub async fn push_to_redis_queue(
    command_id_and_types: &Vec<IdAndType>,
    redis_connection_pool: &Pool<Client>,
) -> Result<(), ()> {
    let redis_conn_result = redis_connection_pool.get();
    if let Err(e) = redis_conn_result {
        println!("{:?}", e);
        return Err(());
    }

    let mut redis_conn = redis_conn_result.unwrap();
    for command_id_and_type in command_id_and_types.iter() {
        let redis_result: redis::RedisResult<()> = redis_conn.lpush(
            command_id_and_type.type_of_task.to_string(),
            command_id_and_type.id,
        );
        if let Err(e) = redis_result {
            println!("{:?}", e);
            return Err(());
        }
    }
    return Ok(());
}
