use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Criterion {
    pub name: String,
    pub point: i32,
}
