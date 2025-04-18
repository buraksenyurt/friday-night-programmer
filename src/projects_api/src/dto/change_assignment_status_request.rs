use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChangeAssignmentStatusRequest {
    pub project_id: u32,
    pub team_id: u32,
    pub status: String,
}
