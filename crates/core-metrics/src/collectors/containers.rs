use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub state: ContainerState,
    pub cpu_usage_percent: f32,
    pub memory_usage_bytes: u64,
    pub memory_limit_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerState {
    Running,
    Paused,
    Stopped,
    Crashed,
    Unknown,
}

impl ContainerInfo {
    pub fn new(
        id: String,
        name: String,
        state: ContainerState,
        cpu_usage_percent: f32,
        memory_usage_bytes: u64,
        memory_limit_bytes: u64,
        network_rx_bytes: u64,
        network_tx_bytes: u64,
        disk_read_bytes: u64,
        disk_write_bytes: u64,
    ) -> Self {
        Self {
            id,
            name,
            state,
            cpu_usage_percent,
            memory_usage_bytes,
            memory_limit_bytes,
            network_rx_bytes,
            network_tx_bytes,
            disk_read_bytes,
            disk_write_bytes,
        }
    }
}