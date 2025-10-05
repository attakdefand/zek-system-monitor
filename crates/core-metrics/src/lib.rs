mod cfg; 
mod ringbuf; 
pub mod collectors;

use crossbeam_channel::{unbounded, Receiver};
use collectors::snapshot::Snapshot;
use std::sync::{Arc, Mutex};
use tracing::debug;

#[derive(Clone)]
pub struct Supervisor { 
    rx: Receiver<Snapshot> 
}

impl Supervisor {
    pub async fn spawn(cfg: cfg::Config) -> anyhow::Result<Self> {
        let (tx, rx) = unbounded();
        let interval = std::time::Duration::from_millis(cfg.refresh.interval_ms);
        
        // Store previous snapshot for calculating deltas
        let previous_snapshot: Arc<Mutex<Option<Snapshot>>> = Arc::new(Mutex::new(None));
        
        tokio::spawn(async move {
            // Create system object with all features enabled
            let mut sys = sysinfo::System::new_all();
            loop {
                // Refresh all system information
                sys.refresh_all();
                
                // Use the associated function instead of method
                let load_avg = sysinfo::System::load_average();
                
                // Get previous snapshot for calculating deltas
                let prev = previous_snapshot.lock().unwrap().clone();
                
                // Debug: Print information about previous snapshot
                if let Some(ref prev_snap) = prev {
                    debug!("Previous snapshot exists with {} network interfaces", prev_snap.network.len());
                    for net in &prev_snap.network {
                        debug!("Previous network data - {}: RX={} TX={}", net.interface, net.rx_bytes, net.tx_bytes);
                    }
                } else {
                    debug!("No previous snapshot");
                }
                
                let snap = collectors::snapshot::Snapshot::from_sysinfo(
                    chrono::Utc::now(), 
                    &sys, 
                    load_avg,
                    prev.as_ref()
                );
                
                // Debug: Print information about current snapshot
                debug!("Current snapshot has {} network interfaces", snap.network.len());
                for net in &snap.network {
                    debug!("Current network data - {}: RX={} TX={} RX_throughput={} TX_throughput={}", 
                              net.interface, net.rx_bytes, net.tx_bytes, net.rx_throughput, net.tx_throughput);
                }
                
                // Store current snapshot for next iteration
                *previous_snapshot.lock().unwrap() = Some(snap.clone());
                
                let _ = tx.send(snap);
                tokio::time::sleep(interval).await;
            }
        });
        
        Ok(Self { rx })
    }
    
    pub fn subscribe(&self) -> Receiver<Snapshot> { 
        self.rx.clone() 
    }
}

pub use cfg::load_cfg;