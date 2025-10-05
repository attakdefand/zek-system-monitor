use serde::{Serialize, Deserialize};
use sysinfo::NetworkData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    // Throughput data (bytes per second)
    pub rx_throughput: f64,
    pub tx_throughput: f64,
}

impl NetworkInfo {
    pub fn from_network_data(name: &str, data: &NetworkData) -> Self {
        Self {
            interface: name.to_string(),
            rx_bytes: data.total_received(),
            tx_bytes: data.total_transmitted(),
            rx_packets: data.total_packets_received(),
            tx_packets: data.total_packets_transmitted(),
            rx_errors: data.total_errors_on_received(),
            tx_errors: data.total_errors_on_transmitted(),
            // Initialize throughput to 0, will be calculated when comparing with previous snapshots
            rx_throughput: 0.0,
            tx_throughput: 0.0,
        }
    }
    
    // Calculate throughput based on previous snapshot
    pub fn with_throughput(mut self, previous: &NetworkInfo, time_delta_ms: f64) -> Self {
        // Calculate bytes per second
        let rx_delta = self.rx_bytes.saturating_sub(previous.rx_bytes);
        let tx_delta = self.tx_bytes.saturating_sub(previous.tx_bytes);
        
        // Convert to bytes per second (time_delta is in milliseconds)
        self.rx_throughput = (rx_delta as f64) / (time_delta_ms / 1000.0);
        self.tx_throughput = (tx_delta as f64) / (time_delta_ms / 1000.0);
        
        self
    }
}