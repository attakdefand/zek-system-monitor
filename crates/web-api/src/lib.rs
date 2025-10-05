use axum::{routing::get, Json, Router, extract::State}; use std::net::SocketAddr;
use core_metrics::Supervisor;

#[derive(Clone)]
struct App { 
    sup: Supervisor 
}

pub async fn serve_api(bind:String,sup:Supervisor)->anyhow::Result<()>{
  let app=Router::new().route("/api/snapshot", get(snapshot)).with_state(App{sup});
  let addr:SocketAddr=bind.parse()?; tracing::info!("API http://{addr}");
  axum::serve(tokio::net::TcpListener::bind(addr).await?,app).await?;
  Ok(())
}

async fn snapshot(State(app):State<App>)->Json<serde_json::Value>{
  let mut last=None; while let Ok(s)=app.sup.subscribe().try_recv(){ last=Some(s); }
  Json(match last{Some(s)=>serde_json::json!(s),None=>serde_json::json!({"status":"no-data-yet"})})
}