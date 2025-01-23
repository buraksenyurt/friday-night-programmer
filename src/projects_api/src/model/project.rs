use crate::model::criteria::Criteria;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub language: String,
    pub summary: String,
    pub criteria_list: Vec<Criteria>,
}
