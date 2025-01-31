use crate::dto::prelude::OperationResponse;
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
        Ok(created) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            "Criteria set has been successfully created.",
            None,
            Some(created),
        )),
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
        Ok(_) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            "Criterion has been successfully added.",
            None,
            Some(criterion),
        )),
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
        Ok(criteria_set) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            format!("'{}' has been successfully retrieved.", criteria_set.name).as_str(),
            None,
            Some(criteria_set),
        )),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
