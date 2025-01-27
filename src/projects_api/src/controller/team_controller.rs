use crate::dto::prelude::UpdateScoresRequest;
use crate::model::prelude::*;
use crate::repository::team_repository::TeamRepository;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

#[post("/teams")]
async fn create_team(team: web::Json<Team>, pool: web::Data<SqlitePool>) -> impl Responder {
    let repository = TeamRepository::new(pool.get_ref().clone());
    match repository.create_team(&team).await {
        Ok(created) => HttpResponse::Ok().json(created),
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
        Ok(inserted) => {
            if inserted > 0 {
                HttpResponse::Ok().json(format!("{} member added successfully.", inserted))
            } else {
                HttpResponse::BadRequest().json("Did not add member to team!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/teams/{id}")]
async fn delete_team(team_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let repository = TeamRepository::new(pool.get_ref().clone());
    match repository.delete_team(*team_id).await {
        Ok(deleted) => {
            if deleted > 0 {
                HttpResponse::Ok().json(format!("{} record deleted successfully", deleted))
            } else {
                HttpResponse::NotFound().json("Did not delete team and members!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/teams")]
async fn update_team_members_scores(
    request: web::Json<UpdateScoresRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = TeamRepository::new(pool.get_ref().clone());
    match repository
        .update_team_members_scores(request.team_id, request.score)
        .await
    {
        Ok(updated) => {
            if updated > 0 {
                HttpResponse::Ok().json(format!("{} Member scores updated successfully", updated))
            } else {
                HttpResponse::NotFound().json("Team mates scores could not be updated.")
            }
        }
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
