use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("api/hello")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Hello world!";
    HttpResponse::Ok().json(json!({"statuc":"sucess", "message": MESSAGE}))
}