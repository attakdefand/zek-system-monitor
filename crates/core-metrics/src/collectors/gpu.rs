use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub usage_percent: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub temperature: f32,
    pub fan_speed_percent: f32,
}

impl GpuInfo {
    pub fn new(
        name: String,
        usage_percent: f32,
        memory_used_bytes: u64,
        memory_total_bytes: u64,
        temperature: f32,
        fan_speed_percent: f32,
    ) -> Self {
        Self {
            name,
            usage_percent,
            memory_used_bytes,
            memory_total_bytes,
            temperature,
            fan_speed_percent,
        }
    }
}