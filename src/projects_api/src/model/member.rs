use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Member {
    #[serde(skip_deserializing)]
    pub id: u32,
    pub identity: String,
    pub full_name: String,
    pub score: i32,
}
