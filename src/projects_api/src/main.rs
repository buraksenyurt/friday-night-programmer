use crate::repository::database;
use actix_web::*;
use dotenvy::dotenv;
use std::env;

mod controller;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = database::setup().await.expect("Failed to set up database");

    env::set_var("RUST_LOG", "actix_web=info,sqlx=debug,projects_api=info");
    env_logger::init();

    let server_address = "0.0.0.0:8080";
    println!("Server is running at https://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(controller::team_controller::create_team)
                    .service(controller::team_controller::add_member_to_team)
                    .service(controller::team_controller::get_team)
                    .service(controller::criteria_controller::create_criteria_set)
                    .service(controller::criteria_controller::add_criterion_to_set)
                    .service(controller::criteria_controller::get_criteria_set)
                    .service(controller::assignment_controller::create_assignment)
                    .service(controller::assignment_controller::get_assignment_by_team)
                    .service(controller::history_controller::create_history),
            )
    })
    .bind(server_address)?
    .run()
    .await?;

    Ok(())
}
