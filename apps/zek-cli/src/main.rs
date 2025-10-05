use anyhow::Result;
use core_metrics::{load_cfg, Supervisor};
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure tracing to only show warnings and errors by default
    tracing_subscriber::fmt()
        .with_env_filter("warn")
        .init();
    
    // Load configuration
    let cfg = load_cfg("configs/zek.toml")?;
    
    // Start the supervisor (metrics collection)
    let supervisor = Supervisor::spawn(cfg.clone()).await?;
    let supervisor = Arc::new(supervisor);
    
    // Clone supervisor for TUI
    let tui_supervisor = supervisor.clone();
    
    // Run TUI in a separate task
    let tui_handle = tokio::spawn(async move {
        tui::run_tui((*tui_supervisor).clone()).await
    });
    
    // Wait for Ctrl+C to shutdown
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Shutting down...");
    
    // Abort the TUI task
    tui_handle.abort();
    
    Ok(())
}