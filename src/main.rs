use std::net::TcpListener;

use zerotooprod::startup::run;
use zerotooprod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to load configuration");
    let app_base = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&app_base)
        .expect("Failed to bind to port");
    println!("App running on {}", &app_base);
    run(listener)?.await
}
