use anyhow::Result; use core_metrics::{load_cfg,Supervisor}; use exporter_prometheus::serve_prometheus; use web_api::serve_api;
#[tokio::main] async fn main()->Result<()>{
  tracing_subscriber::fmt().with_env_filter("info").init();
  let cfg=load_cfg("configs/zek.toml")?; let sup=Supervisor::spawn(cfg.clone()).await?;
  let web_bind=cfg.web.bind.clone().unwrap_or_else(||"127.0.0.1:61208".into());
  let prom_bind=cfg.exporters.prometheus_bind.clone().unwrap_or_else(||"127.0.0.1:9100".into());
  tokio::select!{ r=serve_api(web_bind,sup.clone())=>r?, r=serve_prometheus(prom_bind,sup.clone())=>r?, }; Ok(())
}