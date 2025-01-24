use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub language: String,
    pub summary: String,
    pub criteria_set_id: u32,
}
