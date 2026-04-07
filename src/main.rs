use std::net::TcpListener;

use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use zerotooprod::configuration::get_configuration;
use zerotooprod::startup::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zerotooprod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // `set_global_default` can be used by applications to specify
    //  what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

    let config = get_configuration().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let app_base = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&app_base).expect("Failed to bind to port");
    println!("App running on {}", &app_base);
    run(listener, connection_pool)?.await
}
