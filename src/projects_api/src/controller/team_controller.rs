use crate::model::prelude::*;
use actix_web::*;

#[post("/teams")]
async fn create_team(team: web::Json<Team>) -> impl Responder {
    let response = format!(
        "Team {} created with {} members!",
        team.name,
        team.members.len()
    );
    HttpResponse::Ok().json(response)
}

#[post("/teams/{id}/members")]
async fn add_member_to_team(team_id: web::Path<i32>, member: web::Json<Member>) -> impl Responder {
    let response = format!(
        "Member {} added to team with ID: {}",
        member.full_name, team_id
    );
    HttpResponse::Ok().json(response)
}

#[get("/teams/{id}")]
async fn get_team(team_id: web::Path<i32>) -> impl Responder {
    // Dummy veri dönüyoruz. DB entegrasyonu sonrası değiştiririz.
    let mock_team = Team {
        id: *team_id,
        name: "Team Lorem Ipsum".to_string(),
        members: vec![
            Member {
                identity: "MEM-1234".to_string(),
                full_name: "John Doe".to_string(),
                score: 90,
            },
            Member {
                identity: "MEM-4567".to_string(),
                full_name: "Jane Smith".to_string(),
                score: 95,
            },
        ],
    };
    HttpResponse::Ok().json(mock_team)
}
