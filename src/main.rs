mod config;
mod handlers;

use crate::config::AppConfig;
use crate::handlers::{get_config, get_health_check, openfalcon_push};
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file, if it exists
    dotenv().ok();

    // Initialize the configuration and set it in OnceLock
    let cfg = AppConfig::global();
    tracing::info!("config.toml: {}", cfg);

    // Set default log level if RUST_LOG is not set
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", &cfg.rust_log);
    }

    // Initialize logger
    tracing_subscriber::fmt::init();

    // Initialize server
    let client = Client::new();

    let app = Router::new()
        .route("/health", get(get_health_check))
        .route("/config", get(get_config))
        .route("/v1/push", post(openfalcon_push))
        .route("/opentsdb/put", post(openfalcon_push))
        .route("/openfalcon/push", post(openfalcon_push))
        .route("/prometheus/v1/write", post(openfalcon_push))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind(&cfg.listen_addr)
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
