use actix_web::{HttpResponse, Responder};

// used for testing
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
