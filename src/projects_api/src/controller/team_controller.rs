use crate::model::prelude::*;
use crate::repository::team_repository::TeamRepository;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

#[post("/teams")]
async fn create_team(team: web::Json<Team>, pool: web::Data<SqlitePool>) -> impl Responder {
    let repository = TeamRepository::new(pool.get_ref().clone());
    match repository.create_team(&team).await {
        Ok(_) => HttpResponse::Ok().json("Team created successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/teams/{id}/members")]
async fn add_member_to_team(
    team_id: web::Path<u32>,
    member: web::Json<Member>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = TeamRepository::new(pool.get_ref().clone());
    match repository.add_member_to_team(*team_id, &member).await {
        Ok(_) => HttpResponse::Ok().json("Member added successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/teams/{id}")]
async fn get_team(team_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let repository = TeamRepository::new(pool.get_ref().clone());
    match repository.get_team(*team_id).await {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
