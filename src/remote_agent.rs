// src/remote_agent.rs
//! Remote monitoring agent server (optional feature).
//!
//! Provides HTTP + WebSocket API for remote system monitoring.

#[cfg(feature = "remote")]
use anyhow::Result;

#[cfg(feature = "remote")]
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};

#[cfg(feature = "remote")]
use serde_json::{json, Value};

#[cfg(feature = "remote")]
use std::sync::Arc;

#[cfg(feature = "remote")]
use crate::monitor::{MonitorBackend, SysinfoBackend};

#[cfg(feature = "remote")]
pub struct RemoteAgent {
    backend: Arc<SysinfoBackend>,
    config: RemoteConfig,
}

#[cfg(feature = "remote")]
pub struct RemoteConfig {
    pub bind: String,
    pub port: u16,
    pub token: String,
}

#[cfg(feature = "remote")]
impl RemoteAgent {
    pub fn new(config: RemoteConfig) -> Self {
        Self {
            backend: Arc::new(SysinfoBackend::new()),
            config,
        }
    }
    
    pub async fn start(&self) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/metrics", get(get_metrics))
            .route("/processes", get(get_processes))
            .with_state(self.backend.clone());
        
        let addr = format!("{}:{}", self.config.bind, self.config.port);
        tracing::info!("Remote agent listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

#[cfg(feature = "remote")]
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[cfg(feature = "remote")]
async fn get_metrics(
    State(backend): State<Arc<SysinfoBackend>>,
) -> Result<Json<Value>, StatusCode> {
    let cpu = backend.cpu_snapshot().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let memory = backend.memory_snapshot().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "cpu": cpu,
        "memory": memory,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

#[cfg(feature = "remote")]
async fn get_processes(
    State(backend): State<Arc<SysinfoBackend>>,
) -> Result<Json<Value>, StatusCode> {
    let processes = backend.process_list(None, "cpu", 50).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "processes": processes,
        "count": processes.len(),
    })))
}
