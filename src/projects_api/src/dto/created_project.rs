use serde::Serialize;

#[derive(Serialize)]
pub struct CreatedProject {
    pub id: u32,
    pub name: String,
}
