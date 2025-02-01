use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteCriterionRequest {
    pub set_id: u32,
    pub name: String,
}
