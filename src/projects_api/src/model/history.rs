use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct History {
    pub time: DateTime<Utc>,
    pub event: String,
    pub description: String,
}
