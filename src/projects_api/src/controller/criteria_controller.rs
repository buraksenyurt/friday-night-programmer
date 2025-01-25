use crate::model::prelude::*;
use crate::repository::criteria_repository::CriteriaRepository;
use actix_web::*;
use sqlx::SqlitePool;

#[post("/criteria/set")]
async fn create_criteria_set(
    criteria_set: web::Json<CriteriaSet>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = CriteriaRepository::new(pool.get_ref().clone());
    match repository.create_criteria_set(&criteria_set).await {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/criteria/set/{id}/criterion")]
async fn add_criterion_to_set(
    criteria_set_id: web::Path<u32>,
    criterion: web::Json<Criterion>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = CriteriaRepository::new(pool.get_ref().clone());
    match repository
        .add_criterion_to_criteria_set(*criteria_set_id, &criterion)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Criterion added successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/criteria/set/{id}")]
async fn get_criteria_set(
    criteria_set_id: web::Path<u32>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = CriteriaRepository::new(pool.get_ref().clone());
    match repository.get_criteria_set(*criteria_set_id).await {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
