use crate::dto::prelude::*;
use crate::model::prelude::*;
use crate::repository::assignment_repository::AssignmentRepository;
use actix_web::*;
use sqlx::SqlitePool;

#[post("/assignment")]
async fn create_assignment(
    assignment: web::Json<Assignment>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = AssignmentRepository::new(pool.get_ref().clone());
    match repository.create_assignment(&assignment).await {
        Ok(inserted) => {
            if inserted == 1 {
                HttpResponse::Ok().json("Assignment created")
            } else {
                HttpResponse::BadRequest().json("Assignment did not create!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[patch("/assignment")]
async fn change_assignment_status(
    payload: web::Json<ChangeAssignmentStatusRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = AssignmentRepository::new(pool.get_ref().clone());
    match repository
        .change_assignment_status(
            payload.project_id,
            payload.team_id,
            Status::from(payload.status.as_str().parse().unwrap_or(Status::Planned)),
        )
        .await
    {
        Ok(updated) => {
            if updated > 0 {
                HttpResponse::Ok().json(format!("{} assignment status changed.", updated))
            } else {
                HttpResponse::BadRequest().json("Assignment status did not change!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/assignment")]
async fn get_assignment_by_team(
    payload: web::Json<GetAssignmentRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = AssignmentRepository::new(pool.get_ref().clone());
    match repository
        .get_assignment(payload.project_id, payload.team_id)
        .await
    {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
