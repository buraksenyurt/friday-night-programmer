use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Criteria {
    pub name: String,
    pub point: i32,
}
