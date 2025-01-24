use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CriteriaSet {
    pub id: u32,
    pub name: String,
    pub set: Vec<Criterion>,
}
