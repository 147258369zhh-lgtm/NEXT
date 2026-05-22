use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use std::sync::{Mutex, OnceLock};
use chrono::Local;
use nexus_exec::AppRuntime;

static LAST_ACTIVITY: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_last_activity_mutex() -> &'static Mutex<Option<String>> {
    LAST_ACTIVITY.get_or_init(|| Mutex::new(None))
}

pub fn update_last_activity() {
    let now_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if let Ok(mut guard) = get_last_activity_mutex().lock() {
        *guard = Some(now_str);
    }
}

pub fn get_last_activity() -> Option<String> {
    get_last_activity_mutex().lock().ok()?.clone()
}

#[derive(serde::Serialize, Clone)]
pub struct ConnectorStatus {
    pub id: String,
    pub name: String,
    pub port: u16,
    pub status: String,
    pub last_activity: Option<String>,
}

pub fn list_connectors() -> Vec<ConnectorStatus> {
    vec![
        ConnectorStatus {
            id: "webhook".to_string(),
            name: "Webhook Connector".to_string(),
            port: 3333,
            status: "online".to_string(),
            last_activity: get_last_activity(),
        },
        ConnectorStatus {
            id: "wechat".to_string(),
            name: "WeChat Connector".to_string(),
            port: 18001,
            status: "offline".to_string(),
            last_activity: None,
        },
    ]
}

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
    
    update_last_activity();
    
    match runtime.inject_external_message(payload.message, &payload.source).await {
        Ok(resp) => Json(serde_json::Value::String(resp.reply)),
        Err(err) => Json(serde_json::Value::String(format!("Error: {}", err))),
    }
}
