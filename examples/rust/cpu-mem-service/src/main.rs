use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use std::sync::Mutex;
use sysinfo::System;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        history: Mutex::new(Vec::new()),
    });

    let address = "0.0.0.0:8080";
    println!("Starting server on {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/machine/stats", web::get().to(get_sys_stats))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

struct AppState {
    history: Mutex<Vec<SystemStats>>,
}

#[derive(Serialize, Clone)]
struct SystemStats {
    timestamp: u64,
    cpu_usage: f32,
    memory_total: u64,
    memory_used: u64,
}

async fn get_sys_stats(data: web::Data<AppState>) -> impl Responder {
    let mut system = System::new_all();
    system.refresh_all();

    let cpu_usage = system.global_cpu_usage();
    let memory_total = system.total_memory();
    let memory_used = memory_total - system.available_memory();
    let timestamp = chrono::Utc::now().timestamp() as u64;

    let stats = SystemStats {
        timestamp,
        cpu_usage,
        memory_total,
        memory_used,
    };

    let mut history = data.history.lock().unwrap();
    history.push(stats.clone());
    if history.len() > 50 {
        history.remove(0);
    }

    web::Json(history.clone())
}
