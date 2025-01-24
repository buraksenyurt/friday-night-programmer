use crate::model::prelude::*;
use actix_web::*;
use chrono::{Duration, Utc};
use std::ops::Add;

#[post("/assignment")]
async fn create_assignment(assignment: web::Json<Assignment>) -> impl Responder {
    let response = format!(
        "Assignment to team {} created for project id {}!",
        assignment.team_id, assignment.project_id
    );
    HttpResponse::Ok().json(response)
}

#[get("/assignment/{id}")]
async fn get_assignment_by_team(team_id: web::Path<u32>) -> impl Responder {
    // Dummy veri dönüyoruz. DB entegrasyonu sonrası değiştiririz.
    let now = Utc::now();
    let mock_assignment = Assignment {
        project_id: 1,
        team_id: *team_id,
        status: Status::InProgress,
        start_date: now,
        end_date: now.add(Duration::days(60)),
        repository: "https://github.com/buraksenyurt".to_string(),
    };
    HttpResponse::Ok().json(mock_assignment)
}
