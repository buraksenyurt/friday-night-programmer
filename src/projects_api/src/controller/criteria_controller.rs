use crate::model::prelude::*;
use actix_web::*;
#[post("/criteria/set")]
async fn create_criteria_set(criteria_set: web::Json<CriteriaSet>) -> impl Responder {
    let response = format!(
        "{}, Criteria set '{}' created with {} criteria!",
        criteria_set.id,
        criteria_set.name,
        criteria_set.set.len()
    );
    HttpResponse::Ok().json(response)
}

#[post("/criteria/set/{id}/criterion")]
async fn add_criterion_to_set(
    criteria_set_id: web::Path<u32>,
    criterion: web::Json<Criterion>,
) -> impl Responder {
    let response = format!(
        "Criterion '{}' added to criteria(Id:{}) with {} point",
        criterion.name, criterion.point, criteria_set_id
    );
    HttpResponse::Ok().json(response)
}

#[get("/criteria/set/{id}")]
async fn get_criteria_set(criteria_set_id: web::Path<u32>) -> impl Responder {
    // Dummy veri dönüyoruz. DB entegrasyonu sonrası değiştiririz.
    let mock_criteria_set = CriteriaSet {
        id: *criteria_set_id,
        name: "C# Project Criteria Set".to_string(),
        set: vec![
            Criterion {
                name: "Use of OOP principles".to_string(),
                point: 30,
            },
            Criterion {
                name: "Originality".to_string(),
                point: 5,
            },
            Criterion {
                name: "Model design".to_string(),
                point: 5,
            },
            Criterion {
                name: "Clean code factors".to_string(),
                point: 20,
            },
            Criterion {
                name: "Warning counts".to_string(),
                point: 15,
            },
            Criterion {
                name: "Memory consumption".to_string(),
                point: 5,
            },
            Criterion {
                name: "User experience".to_string(),
                point: 5,
            },
            Criterion {
                name: "Tooling".to_string(),
                point: 5,
            },
            Criterion {
                name: "Source code repo check".to_string(),
                point: 10,
            },
        ],
    };
    HttpResponse::Ok().json(mock_criteria_set)
}
