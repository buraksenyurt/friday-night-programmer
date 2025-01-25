use serde::Serialize;

#[derive(Serialize)]
pub struct CreatedCriteria {
    pub id: u32,
    pub name: String,
    pub criteria_count: usize,
}
