use crate::dto::prelude::*;
use crate::repository::member_repository::MemberRepository;
use actix_web::{put, web, HttpResponse, Responder};
use sqlx::SqlitePool;

#[put("/members")]
async fn move_member_to_another_team(
    request: web::Json<MoveMemberRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = MemberRepository::new(pool.get_ref().clone());
    match repository
        .move_member_to_another_team(request.member_id, request.team_id)
        .await
    {
        Ok(updated) => {
            if updated > 0 {
                HttpResponse::Ok().json(format!("{} Member moved to a new team", updated))
            } else {
                HttpResponse::NotFound().json("Member moved to a team did not completed!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
