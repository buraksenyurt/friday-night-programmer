use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Default, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Planned,
    InProgress,
    Completed,
    Failed,
    Unknown,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Planned => "Planned".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Completed => "Completed".to_string(),
            Status::Failed => "Failed".to_string(),
            Status::Unknown => "Unknown".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for Status {
    fn from(str: &str) -> Self {
        match str {
            "Planned" => Status::Planned,
            "InProgress" => Status::InProgress,
            "Completed" => Status::Completed,
            "Failed" => Status::Failed,
            _ => Status::Unknown,
        }
    }
}
