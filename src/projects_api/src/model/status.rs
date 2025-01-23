use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Status {
    Planned,
    InProgress,
    Completed,
    Failed,
}
