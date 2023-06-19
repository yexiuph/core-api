use actix_web::middleware::Logger; // If we need to log.
use actix_web::{App, HttpServer};
use core_api::services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Core : DAS has started successfully.");

    HttpServer::new(move || {
        App::new()
            .service(health_check::health_check_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
