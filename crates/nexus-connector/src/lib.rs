use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use nexus_exec::AppRuntime;

#[derive(Debug, Deserialize)]
pub struct ExternalMessage {
    pub message: String,
    pub source: String,
}

pub struct ConnectorConfig {
    pub port: u16,
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        Self { port: 3333 }
    }
}

pub async fn start_connector_server(
    runtime: Arc<AppRuntime>,
    config: ConnectorConfig,
) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/webhook/incoming", post(handle_webhook))
        .with_state(runtime);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("🚀 Connector server listening on {}", addr);
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_webhook(
    State(runtime): State<Arc<AppRuntime>>,
    Json(payload): Json<ExternalMessage>,
) -> Json<serde_json::Value> {
    println!("📩 Received external message from {}: {}", payload.source, payload.message);
    
    match runtime.inject_external_message(payload.message, &payload.source).await {
        Ok(resp) => Json(serde_json::Value::String(resp.reply)),
        Err(err) => Json(serde_json::Value::String(format!("Error: {}", err))),
    }
}
