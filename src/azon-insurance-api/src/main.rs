use actix_web::{web, App, HttpServer};
use crate::service::Service;

mod customer;
mod service;
mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "0.0.0.0:8080";
    println!("Server is running at http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .route("/customers",web::get().to(Service::get_customers))
    })
        .bind(server_address)?
        .run()
        .await?;

    Ok(())
}
