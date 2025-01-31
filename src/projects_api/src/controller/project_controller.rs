use crate::dto::prelude::OperationResponse;
use crate::model::prelude::*;
use crate::repository::project_repository::ProjectRepository;
use actix_web::*;
use sqlx::SqlitePool;

#[post("/projects")]
async fn create_project(
    project: web::Json<Project>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = ProjectRepository::new(pool.get_ref().clone());
    match repository.create_project(&project).await {
        Ok(created) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            "Project created",
            None,
            Some(created),
        )),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/projects/{id}")]
async fn get_project_by_id(
    project_id: web::Path<u32>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let repository = ProjectRepository::new(pool.get_ref().clone());
    match repository.get_project(*project_id).await {
        Ok(project_with_criteria) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            format!(
                "'{}' has been retrieved.",
                project_with_criteria.project.name
            )
            .as_str(),
            None,
            Some(project_with_criteria),
        )),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
