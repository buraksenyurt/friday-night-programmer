use crate::model::member::Member;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub members: Vec<Member>,
}
