mod cfg; 
mod ringbuf; 
pub mod collectors;

use crossbeam_channel::{unbounded, Receiver};
use collectors::snapshot::Snapshot;
use std::sync::{Arc, Mutex};
use tracing::debug;
use std::collections::VecDeque;
use std::time::Duration;

// Historical data storage
pub struct HistoricalData {
    data_points: Arc<Mutex<VecDeque<Snapshot>>>,
    max_points: usize,
}

impl HistoricalData {
    pub fn new(max_points: usize) -> Self {
        Self {
            data_points: Arc::new(Mutex::new(VecDeque::with_capacity(max_points))),
            max_points,
        }
    }

    pub fn add_snapshot(&self, snapshot: Snapshot) {
        let mut data = self.data_points.lock().unwrap();
        if data.len() >= self.max_points {
            data.pop_front();
        }
        data.push_back(snapshot);
    }

    pub fn get_recent_data(&self, duration_secs: u64) -> Vec<Snapshot> {
        let data = self.data_points.lock().unwrap();
        let cutoff_time = chrono::Utc::now().timestamp_millis() - (duration_secs as i64 * 1000);
        
        data.iter()
            .filter(|snapshot| snapshot.ts >= cutoff_time)
            .cloned()
            .collect()
    }

    pub fn get_all_data(&self) -> Vec<Snapshot> {
        let data = self.data_points.lock().unwrap();
        data.iter().cloned().collect()
    }
}

#[derive(Clone)]
pub struct Supervisor { 
    rx: Receiver<Snapshot>,
    historical_data: Arc<HistoricalData>,
}

impl Supervisor {
    pub async fn spawn(cfg: cfg::Config) -> anyhow::Result<Self> {
        let (tx, rx) = unbounded();
        let interval = std::time::Duration::from_millis(cfg.refresh.interval_ms);
        
        // Historical data storage (last 1 hour of data)
        let historical_data = Arc::new(HistoricalData::new(3600)); // 3600 points = 1 hour at 1 second intervals
        let historical_data_clone = historical_data.clone();
        
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
                
                // Store in historical data
                historical_data_clone.add_snapshot(snap.clone());
                
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
        
        Ok(Self { rx, historical_data })
    }
    
    pub fn subscribe(&self) -> Receiver<Snapshot> { 
        self.rx.clone() 
    }
    
    pub fn get_historical_data(&self, duration_secs: u64) -> Vec<Snapshot> {
        self.historical_data.get_recent_data(duration_secs)
    }
    
    pub fn get_all_historical_data(&self) -> Vec<Snapshot> {
        self.historical_data.get_all_data()
    }
}

pub use cfg::load_cfg;