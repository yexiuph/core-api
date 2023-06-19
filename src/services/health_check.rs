use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/api/healthcheck")]
pub async fn health_check_handler() -> impl Responder {
    // We would change this later to a common response with status and msg that is corresponding.
    const MSG : &str = "Thank you for the health check.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MSG
    }))
}