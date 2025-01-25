use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetAssignmentRequest {
    pub project_id: u32,
    pub team_id: u32,
}
