use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Default, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Planned,
    InProgress,
    Completed,
    Failed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Status::Planned => "Planned".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Completed => "Completed".to_string(),
            Status::Failed => "Failed".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Planned" => Ok(Status::Planned),
            "InProgress" => Ok(Status::InProgress),
            "Completed" => Ok(Status::Completed),
            "Failed" => Ok(Status::Failed),
            _ => Err(format!("Invalid status value: {}", s)),
        }
    }
}
