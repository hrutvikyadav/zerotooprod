use actix_web::{HttpResponse, Responder, http::StatusCode};

pub async fn health_check() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
}
