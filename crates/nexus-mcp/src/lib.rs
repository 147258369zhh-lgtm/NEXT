use std::process::Stdio;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use tokio::sync::oneshot;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcNotification {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerDescriptor {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

pub struct McpClient {
    descriptor: McpServerDescriptor,
    inner: Arc<Mutex<McpClientInner>>,
}

struct McpClientInner {
    process: Option<Child>,
    pending_requests: HashMap<String, oneshot::Sender<Result<Value, String>>>,
    next_id: u64,
}

impl McpClient {
    pub fn new(descriptor: McpServerDescriptor) -> Self {
        Self {
            descriptor,
            inner: Arc::new(Mutex::new(McpClientInner {
                process: None,
                pending_requests: HashMap::new(),
                next_id: 1,
            })),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let mut child = Command::new(&self.descriptor.command)
            .args(&self.descriptor.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn MCP server process")?;

        let stdout = child.stdout.take().context("No stdout")?;
        let inner_clone = Arc::clone(&self.inner);

        // 启动后台读取循环
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Ok(res) = serde_json::from_str::<JsonRpcResponse>(&line) {
                    let mut inner = inner_clone.lock().await;
                    if let Some(id) = res.id.as_str() {
                        if let Some(tx) = inner.pending_requests.remove(id) {
                            if let Some(error) = res.error {
                                let _ = tx.send(Err(error.to_string()));
                            } else {
                                let _ = tx.send(Ok(res.result.unwrap_or(Value::Null)));
                            }
                        }
                    }
                }
            }
        });

        {
            let mut inner = self.inner.lock().await;
            inner.process = Some(child);
        }

        // 发送 initialize
        self.call_method("initialize", json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "nexus-agent", "version": "0.1.0" }
        })).await?;

        // 发送 initialized 通知
        self.send_notification("notifications/initialized", None).await?;

        Ok(())
    }

    async fn call_method(&self, method: &str, params: Value) -> Result<Value> {
        let (tx, rx) = oneshot::channel();
        let id = {
            let mut inner = self.inner.lock().await;
            let id = inner.next_id.to_string();
            inner.next_id += 1;
            inner.pending_requests.insert(id.clone(), tx);
            
            let req = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: Value::String(id.clone()),
                method: method.to_string(),
                params,
            };
            
            let json = serde_json::to_string(&req)? + "\n";
            if let Some(process) = &mut inner.process {
                if let Some(stdin) = &mut process.stdin {
                    stdin.write_all(json.as_bytes()).await?;
                    stdin.flush().await?;
                }
            }
            id
        };

        rx.await.map_err(|_| anyhow::anyhow!("Request dropped"))?
            .map_err(|e| anyhow::anyhow!("RPC Error: {}", e))
    }

    async fn send_notification(&self, method: &str, params: Option<Value>) -> Result<()> {
        let mut inner = self.inner.lock().await;
        let notif = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        };
        let json = serde_json::to_string(&notif)? + "\n";
        if let Some(process) = &mut inner.process {
            if let Some(stdin) = &mut process.stdin {
                stdin.write_all(json.as_bytes()).await?;
                stdin.flush().await?;
            }
        }
        Ok(())
    }

    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let res = self.call_method("tools/list", json!({})).await?;
        let tools: Vec<McpTool> = serde_json::from_value(res["tools"].clone())
            .context("Failed to parse tools list")?;
        Ok(tools)
    }

    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        self.call_method("tools/call", json!({
            "name": name,
            "arguments": arguments
        })).await
    }
}
