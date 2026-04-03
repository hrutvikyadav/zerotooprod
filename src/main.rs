use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zerotooprod::configuration::get_configuration;
use zerotooprod::startup::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // init calls set_logger implicitly
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_configuration().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let app_base = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&app_base).expect("Failed to bind to port");
    println!("App running on {}", &app_base);
    run(listener, connection_pool)?.await
}
