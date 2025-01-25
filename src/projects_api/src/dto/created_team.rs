use serde::Serialize;

#[derive(Serialize)]
pub struct CreatedTeam {
    pub id: u32,
    pub name: String,
    pub member_count: usize,
}
