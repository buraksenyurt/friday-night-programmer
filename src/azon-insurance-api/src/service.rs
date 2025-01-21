use actix_web::{HttpResponse, Responder};
use crate::data::generate_customers;

pub struct Service {}

impl Service {
    pub async fn get_customers() -> impl Responder {
        let customers=generate_customers();
        HttpResponse::Ok().json(customers)
    }
}