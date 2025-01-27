use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateScoresRequest {
    pub team_id: u32,
    pub score: u32,
}
