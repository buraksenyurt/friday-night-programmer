use crate::model::prelude::*;
use actix_web::*;

#[post("/projects")]
async fn create_project(project: web::Json<Project>) -> impl Responder {
    let response = format!(
        "Project '{}' created with criteria set '{}'!",
        project.name, project.criteria_set_id
    );
    HttpResponse::Ok().json(response)
}

#[get("/projects/{id}")]
async fn get_project_by_id(project_id: web::Path<u32>) -> impl Responder {
    // Dummy veri dönüyoruz. DB entegrasyonu sonrası değiştiririz.
    let mock_project = Project {
        id: *project_id,
        name: "Terminal-Based Dungeon Game".to_string(),
        language: "C#".to_string(),
        summary: "Terminalden programın sorduğu sorulara göre oyuncuyu yönlendirilen \
        bir zindan oyunudur. \
        Tek level tasarlanması yeterlidir. Görsel bir öğe içermemektedir."
            .to_string(),
        criteria_set_id: 1,
    };
    HttpResponse::Ok().json(mock_project)
}
