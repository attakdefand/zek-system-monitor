use serde::{Serialize, Deserialize};
use sysinfo::Disk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub usage_percent: f32,
}

impl DiskInfo {
    pub fn from_disk(disk: &Disk) -> Self {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let usage = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        // On Windows, disk.name() might be empty, so use mount_point as fallback
        let name = disk.name().to_string_lossy().to_string();
        let mount_point = disk.mount_point().to_string_lossy().to_string();
        let display_name = if name.is_empty() { 
            mount_point.clone() 
        } else { 
            name 
        };
        
        Self {
            name: display_name,
            mount_point,
            total_space: total,
            available_space: available,
            used_space: used,
            usage_percent: usage,
        }
    }
}