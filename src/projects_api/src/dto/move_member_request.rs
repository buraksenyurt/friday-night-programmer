use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveMemberRequest {
    pub member_id: u32,
    pub team_id: u32,
}
