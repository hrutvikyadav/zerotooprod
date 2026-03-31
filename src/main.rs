use std::net::TcpListener;

use zerotooprod::startup::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    const APP_BASE: &str = "127.0.0.1:8050";
    let listener = TcpListener::bind(APP_BASE)
        .expect("Failed to bind to port");
    println!("App running on {}", APP_BASE);
    run(listener)?.await
}
