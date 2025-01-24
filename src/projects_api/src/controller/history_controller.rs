use crate::model::prelude::*;
use actix_web::*;

#[post("/history")]
async fn create_history(history: web::Json<History>) -> impl Responder {
    // history.time = Utc::now();
    // history db'ye yazılırken burada zaman damgasını otomatik üretebiliriz

    let response = format!("Created history event '{}'", history.event);
    HttpResponse::Ok().json(response)
}
