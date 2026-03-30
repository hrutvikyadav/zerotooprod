use actix_web::{App, HttpResponse, HttpServer, Responder, http::StatusCode, web};

async fn health_check() -> impl Responder {
    HttpResponse::new(StatusCode::OK)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8050")?
    .run()
    .await
}
