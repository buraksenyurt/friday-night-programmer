use actix_web::*;
use std::env;

mod controller;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server_address = "0.0.0.0:8080";
    println!("Server is running at https://{}", server_address);

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api")
                .service(controller::team_controller::create_team)
                .service(controller::team_controller::add_member_to_team)
                .service(controller::team_controller::get_team)
                .service(controller::criteria_controller::create_criteria_set)
                .service(controller::criteria_controller::add_criterion_to_set)
                .service(controller::criteria_controller::get_criteria_set),
        )
    })
    .bind(server_address)?
    .run()
    .await?;

    Ok(())
}
