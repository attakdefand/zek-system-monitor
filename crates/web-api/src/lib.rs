use axum::{
    routing::{get, post},
    Json, Router, extract::State,
    http::StatusCode,
    response::IntoResponse
};
use std::net::SocketAddr;
use core_metrics::Supervisor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
struct App { 
    sup: Supervisor 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertConfig {
    pub metric: String,
    pub threshold: f64,
    pub operator: String, // "gt", "lt", "eq"
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoricalDataPoint {
    pub timestamp: i64,
    pub metric: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportRequest {
    pub format: String, // "csv", "json"
    pub metrics: Vec<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

pub async fn serve_api(bind:String,sup:Supervisor)->anyhow::Result<()>{
    let app = Router::new()
        .route("/api/snapshot", get(snapshot))
        .route("/api/history", get(get_historical_data))
        .route("/api/alerts", get(get_alerts).post(create_alert))
        .route("/api/export", post(export_data))
        .route("/api/trends", get(get_trends))
        .with_state(App{sup});
        
    let addr:SocketAddr=bind.parse()?; 
    tracing::info!("API http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?,app).await?;
    Ok(())
}

async fn snapshot(State(app):State<App>)->Json<serde_json::Value>{
    let mut last=None; 
    while let Ok(s)=app.sup.subscribe().try_recv(){ 
        last=Some(s); 
    }
    Json(match last{
        Some(s)=>serde_json::json!(s),
        None=>serde_json::json!({"status":"no-data-yet"})
    })
}

async fn get_historical_data(State(_app): State<App>) -> impl IntoResponse {
    // Return mock historical data for now
    let data = vec![
        HistoricalDataPoint {
            timestamp: chrono::Utc::now().timestamp_millis(),
            metric: "cpu_usage".to_string(),
            value: 45.5,
        }
    ];
    
    (StatusCode::OK, Json(data))
}

async fn get_alerts(State(_app): State<App>) -> impl IntoResponse {
    // Return mock alert configs for now
    let alerts: HashMap<String, AlertConfig> = HashMap::new();
    (StatusCode::OK, Json(alerts))
}

async fn create_alert(State(_app): State<App>, Json(_alert): Json<AlertConfig>) -> impl IntoResponse {
    // Mock implementation
    (StatusCode::CREATED, Json(serde_json::json!({"status": "Alert created"})))
}

async fn export_data(State(_app): State<App>, Json(_request): Json<ExportRequest>) -> impl IntoResponse {
    // Mock implementation
    (StatusCode::OK, Json(serde_json::json!({"status": "Export started", "job_id": "12345"})))
}

async fn get_trends(State(_app): State<App>) -> impl IntoResponse {
    // Return mock trend data for now
    let trends = serde_json::json!({
        "cpu_trend": "stable",
        "memory_trend": "increasing",
        "network_trend": "decreasing"
    });
    
    (StatusCode::OK, Json(trends))
}