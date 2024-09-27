use crate::adapters::aws_config::AwsConfig;
use crate::api::app_config::AppConfig;
use crate::api::routes::{configure_routes, AppState};
use actix_web::{web, App, HttpServer};
use log::info;

mod adapters;
mod api;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("Starting ODO Orchestrator API...");

    let aws_config = AwsConfig::new();
    let app_config = AppConfig::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                aws_config: aws_config.clone(),
                app_config: app_config.clone(),
            }))
            .configure(configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
