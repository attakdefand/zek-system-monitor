use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorInfo {
    pub component: String,
    pub temperature: f32,
    pub unit: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub name: String,
    pub charge_percent: f32,
    pub health_percent: f32,
    pub state: BatteryState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    Unknown,
}

impl BatteryInfo {
    pub fn new(name: String, charge_percent: f32, health_percent: f32, state: BatteryState) -> Self {
        Self {
            name,
            charge_percent,
            health_percent,
            state,
        }
    }
}