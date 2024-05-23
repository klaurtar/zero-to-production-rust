use actix_web::{HttpResponse, HttpServer};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
