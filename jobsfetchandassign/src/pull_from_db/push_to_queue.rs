use r2d2::Pool;
use redis::Client;

use crate::models::tasks::IdAndType;

pub async fn push_to_redis_queue(
    command_id_and_type: &Vec<IdAndType>,
    redis_connection_pool: &Pool<Client>,
) -> Result<(), ()> {
    return Ok(());
}
