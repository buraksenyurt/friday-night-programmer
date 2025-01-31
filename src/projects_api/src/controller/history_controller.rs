use crate::dto::prelude::OperationResponse;
use crate::repository::history_repository::HistoryRepository;
use actix_web::*;
use sqlx::SqlitePool;

#[get("/history")]
async fn get_history(pool: web::Data<SqlitePool>) -> impl Responder {
    let repository = HistoryRepository::new(pool.get_ref().clone());
    match repository.get_history().await {
        Ok(history_list) => HttpResponse::Ok().json(OperationResponse::new(
            true,
            format!("'{}' item has been retrieved", history_list.len()).as_str(),
            None,
            Some(history_list),
        )),
        Err(err) => HttpResponse::InternalServerError().json(OperationResponse::new(
            false,
            err.to_string().as_str(),
            None,
            None::<()>,
        )),
    }
}
