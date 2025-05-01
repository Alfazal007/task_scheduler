use crate::models::tasks::IdAndType;

pub async fn push_to_redis_queue(command_id_and_type: &Vec<IdAndType>) -> Result<(), ()> {
    return Ok(());
}
