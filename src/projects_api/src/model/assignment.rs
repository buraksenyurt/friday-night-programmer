use crate::model::status::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    pub project_id: u32,
    pub team_id: u32,
    #[serde(skip_deserializing)]
    pub status: Status,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub repository: String,
}
