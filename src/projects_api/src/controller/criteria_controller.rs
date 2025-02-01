use crate::dto::prelude::{DeleteCriterionRequest, OperationResponse};
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
        Err(err) => HttpResponse::InternalServerError().json(OperationResponse::new(
            false,
            err.to_string().as_str(),
            None,
            None::<()>,
        )),
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
        Err(err) => HttpResponse::InternalServerError().json(OperationResponse::new(
            false,
            err.to_string().as_str(),
            None,
            None::<()>,
        )),
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
        Err(err) => HttpResponse::InternalServerError().json(OperationResponse::new(
            false,
            err.to_string().as_str(),
            None,
            None::<()>,
        )),
    }
}

#[get("/criteria")]
async fn get_all_criteria(pool: web::Data<SqlitePool>) -> impl Responder {
    let repository = CriteriaRepository::new(pool.get_ref().clone());
    match repository.get_all_criteria().await {
        Ok(criteria_sets) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            format!(
                "'{}' criteria has been successfully retrieved.",
                criteria_sets.len()
            )
            .as_str(),
            None,
            Some(criteria_sets),
        )),
        Err(err) => HttpResponse::InternalServerError().json(OperationResponse::new(
            false,
            err.to_string().as_str(),
            None,
            None::<()>,
        )),
    }
}

#[delete("/criterion")]
async fn delete_criterion(
    request: web::Json<DeleteCriterionRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = CriteriaRepository::new(pool.get_ref().clone());
    let name = request.name.clone();
    match repository.delete_criterion(request.set_id, name).await {
        Ok(deleted) => {
            if deleted > 0 {
                HttpResponse::Ok().json(OperationResponse::new(
                    true,
                    format!("{} record deleted successfully", deleted).as_str(),
                    None,
                    None::<()>,
                ))
            } else {
                HttpResponse::NotFound().json(OperationResponse::new(
                    false,
                    "Did not delete criterion!",
                    None,
                    None::<()>,
                ))
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(OperationResponse::new(
            false,
            err.to_string().as_str(),
            None,
            None::<()>,
        )),
    }
}
