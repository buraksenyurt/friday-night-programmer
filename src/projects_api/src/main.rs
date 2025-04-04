use crate::controller::*;
use crate::repository::database;
use actix_web::*;
use dotenvy::dotenv;
use std::env;

mod controller;
mod dto;
mod enums;
mod model;
mod repository;
mod utility;

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
                    .service(team_controller::create_team)
                    .service(team_controller::add_member_to_team)
                    .service(team_controller::get_team)
                    .service(team_controller::update_team_members_scores)
                    .service(team_controller::delete_team)
                    .service(criteria_controller::create_criteria_set)
                    .service(criteria_controller::add_criterion_to_set)
                    .service(criteria_controller::get_criteria_set)
                    .service(criteria_controller::get_all_criteria)
                    .service(criteria_controller::delete_criterion)
                    .service(history_controller::get_history)
                    .service(assignment_controller::create_assignment)
                    .service(assignment_controller::get_assignment_by_team)
                    .service(assignment_controller::change_assignment_status)
                    .service(project_controller::create_project)
                    .service(project_controller::get_project_by_id)
                    .service(member_controller::move_member_to_another_team),
            )
    })
    .bind(server_address)?
    .run()
    .await?;

    Ok(())
}
