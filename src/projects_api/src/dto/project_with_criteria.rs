use crate::model::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectWithCriteria {
    pub project: Project,
    pub criteria_set: CriteriaSet,
}
