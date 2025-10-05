use anyhow::Result; use axum::{routing::get, Router}; use std::net::SocketAddr;
#[tokio::main] async fn main()->Result<()>{
  tracing_subscriber::fmt().with_env_filter("info").init();
  let app=Router::new().route("/", get(|| async{"zek-gateway (stub)"})); let addr:SocketAddr="127.0.0.1:7001".parse()?;
  tracing::info!("Gateway http://{addr}"); axum::serve(tokio::net::TcpListener::bind(addr).await?,app).await?; Ok(())
}