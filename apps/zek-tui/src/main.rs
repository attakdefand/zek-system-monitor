use anyhow::Result; use core_metrics::{load_cfg,Supervisor}; use tui::run_tui;
#[tokio::main] async fn main()->Result<()>{
  tracing_subscriber::fmt().with_env_filter("info").init();
  let cfg=load_cfg("configs/zek.toml")?; let sup=Supervisor::spawn(cfg).await?; run_tui(sup).await
}