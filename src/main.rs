use actix_web::middleware::Logger; // If we need to log.
use actix_web::{web, App, HttpServer};
use sqlx::mssql::{ MssqlPool, MssqlPoolOptions };
use dotenv::dotenv;
use core_api::{
    services::*,
};

pub struct AppState {
    database: MssqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // Adding Test for MSSQL Connection before we add it to the utils folder.
    // Trying to connect lol. But unable to.
    // Will continue in the next video.
    dotenv().ok();
    let db_conn = std::env::var("DB_CONNECTION").expect("Database must be correct and set.");
    let db_pool = match MssqlPoolOptions::new()
        .max_connections(10)
        .connect(&db_conn)
        .await
    {
        Ok(db_pool) => {
            println!("Connected to the database.");
            db_pool
        }
        Err(err) => {
            println!("Failed to connect to the database : {:?}", err);
            std::process::exit(1);
        }
    };

    // TODO! : Check the cause of timeout in the MSSQL Connection. - YeXiuPH
    
    println!("Core : DAS has started successfully.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { database: db_pool.clone() }))
            .service(health_check::health_check_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
