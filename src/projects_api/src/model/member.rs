use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Member {
    pub identity: String,
    pub full_name: String,
    pub score: i32,
}
