#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "TypeOfTask")]
pub enum TypeOfTask {
    BASH,
    DOCKER,
    NODE,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub command: String,
    #[sqlx(rename = "scheduledAt")]
    pub scheduled_at: i64,
    #[sqlx(rename = "typeOfTask")]
    pub type_of_task: TypeOfTask,
    #[sqlx(rename = "pickedAt")]
    pub picked_at: i64,
    #[sqlx(rename = "completedAt")]
    pub completed_at: i64,
    #[sqlx(rename = "startedAt")]
    pub started_at: i64,
    #[sqlx(rename = "failedAt")]
    pub failed_at: i64,
}
