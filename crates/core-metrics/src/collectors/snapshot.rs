use serde::{Serialize,Deserialize};
use super::{net::NetworkInfo, disk::DiskInfo, process::ProcessInfo, sensors::{SensorInfo, BatteryInfo}, gpu::GpuInfo, connections::ConnectionInfo, process_tree::ProcessTreeNode, containers::ContainerInfo};
use std::collections::HashMap;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Snapshot{
    pub ts:i64,
    pub cpu_total_pct:f64,
    pub cpu_per_core: Vec<f32>,  // New: per-core CPU usage
    pub mem_used_bytes:u64,
    pub mem_total_bytes:u64,
    pub swap_used_bytes:u64,     // New: swap memory usage
    pub swap_total_bytes:u64,    // New: swap memory total
    pub load1:f64,
    pub load5:f64,
    pub load15:f64,
    pub network: Vec<NetworkInfo>,
    pub disks: Vec<DiskInfo>,
    pub top_processes: Vec<ProcessInfo>,
    // New fields for medium-term goals
    pub sensors: Vec<SensorInfo>,
    pub batteries: Vec<BatteryInfo>,
    pub gpus: Vec<GpuInfo>,
    pub connections: Vec<ConnectionInfo>,
    pub process_tree: Vec<ProcessTreeNode>,
    pub containers: Vec<ContainerInfo>,
}

impl Snapshot{
  pub fn from_sysinfo(now:chrono::DateTime<chrono::Utc>, sys:&sysinfo::System, load_avg: sysinfo::LoadAvg, previous: Option<&Snapshot>)->Self{
    // Collect per-core CPU usage
    let cpus = sys.cpus();
    let cpu_per_core: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();
    let cpu_total = if cpus.is_empty() { 
        0.0 
    } else { 
        cpus.iter().map(|c| c.cpu_usage() as f64).sum::<f64>() / cpus.len() as f64 
    };
    
    // Collect memory information
    let total_mem = sys.total_memory(); 
    let avail_mem = sys.available_memory(); 
    let used_mem = total_mem.saturating_sub(avail_mem);
    
    // Collect swap memory information
    let total_swap = sys.total_swap();
    let free_swap = sys.free_swap();
    let used_swap = total_swap.saturating_sub(free_swap);
    
    // Collect real network data - in sysinfo 0.30, networks are separate objects
    let networks = sysinfo::Networks::new_with_refreshed_list();
    
    // Calculate network throughput if we have previous data
    let network: Vec<NetworkInfo> = if let Some(prev) = previous {
        let time_delta_ms = (now.timestamp_millis() - prev.ts) as f64;
        
        // Create a map of previous network data for easy lookup
        let mut prev_network_map: HashMap<String, &NetworkInfo> = HashMap::new();
        for net in &prev.network {
            prev_network_map.insert(net.interface.clone(), net);
        }
        
        networks
            .iter()
            .map(|(name, data)| {
                let base_info = NetworkInfo::from_network_data(name, data);
                // If we have previous data for this interface, calculate throughput
                if let Some(prev_net) = prev_network_map.get(name) {
                    base_info.with_throughput(prev_net, time_delta_ms)
                } else {
                    base_info
                }
            })
            .collect()
    } else {
        // No previous data, just collect raw network data
        networks
            .iter()
            .map(|(name, data)| NetworkInfo::from_network_data(name, data))
            .collect()
    };
    
    // Collect real disk data - in sysinfo 0.30, disks are separate objects
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let disks: Vec<DiskInfo> = disks
        .iter()
        .map(|disk| DiskInfo::from_disk(disk))
        .collect();
    
    // Collect sensor information
    let components = sysinfo::Components::new_with_refreshed_list();
    let sensors: Vec<SensorInfo> = components
        .iter()
        .map(|component| {
            SensorInfo {
                component: component.label().to_string(),
                temperature: component.temperature(),
                unit: "°C".to_string(),
                label: component.label().to_string(),
            }
        })
        .collect();
    
    // Collect battery information - sysinfo doesn't directly provide battery info
    // For now, we'll leave this as an empty vector, but in a complete implementation
    // we would use platform-specific APIs or crates like `battery` crate
    let batteries: Vec<BatteryInfo> = Vec::new();
    
    // Collect real process data (top 10 by CPU usage)
    let mut processes: Vec<(&sysinfo::Pid, &sysinfo::Process)> = sys
        .processes()
        .iter()
        .collect();
    
    // Sort by CPU usage (descending)
    processes.sort_by(|a, b| {
        b.1.cpu_usage()
            .partial_cmp(&a.1.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Take top 10 processes
    let top_processes: Vec<ProcessInfo> = processes
        .into_iter()
        .take(10)
        .map(|(pid, process)| ProcessInfo::from_process(pid.as_u32(), process))
        .collect();
    
    // Build process tree from the process data
    let process_tree: Vec<ProcessTreeNode> = build_process_tree(&sys);
    
    // Collect GPU information - sysinfo doesn't directly provide GPU info
    // For now, we'll leave this as an empty vector, but in a complete implementation
    // we would use platform-specific APIs or crates like `nvml` for NVIDIA GPUs
    let gpus: Vec<GpuInfo> = Vec::new();
    
    // Collect connection information
    let connections: Vec<ConnectionInfo> = super::connections::collect_connections();
    
    // Collect container information - sysinfo doesn't directly provide container info
    // For now, we'll leave this as an empty vector, but in a complete implementation
    // we would use Docker API or container runtime APIs
    let containers: Vec<ContainerInfo> = Vec::new();

    Self{
        ts:now.timestamp_millis(),
        cpu_total_pct:cpu_total,
        cpu_per_core,
        mem_used_bytes:used_mem * 1024,
        mem_total_bytes:total_mem * 1024,
        swap_used_bytes:used_swap * 1024,
        swap_total_bytes:total_swap * 1024,
        load1:load_avg.one,
        load5:load_avg.five,
        load15:load_avg.fifteen,
        network,
        disks,
        top_processes,
        // Initialize new fields - sensors are now collected, others remain empty for now
        sensors,
        batteries,
        gpus,
        connections,
        process_tree,
        containers,
    }
  }
}

fn build_process_tree(sys: &sysinfo::System) -> Vec<ProcessTreeNode> {
    let processes = sys.processes();
    
    // Create a map of all processes for easy lookup
    let mut process_map: std::collections::HashMap<u32, ProcessTreeNode> = std::collections::HashMap::new();
    let mut parent_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    
    // First, create ProcessTreeNode for each process and track parent relationships
    for (pid, process) in processes {
        let pid_u32 = pid.as_u32();
        let parent_pid = process.parent().map(|p| p.as_u32());
        
        let node = ProcessTreeNode::new(
            pid_u32,
            process.name().to_string(),
            process.cpu_usage(),
            process.memory(),
            parent_pid,
        );
        
        process_map.insert(pid_u32, node);
        
        if let Some(parent_pid) = parent_pid {
            parent_map.insert(pid_u32, parent_pid);
        }
    }
    
    // Then, build the tree structure by assigning children to their parents
    for (child_pid, parent_pid) in parent_map {
        if let (Some(child_node), Some(parent_node)) = (process_map.remove(&child_pid), process_map.get_mut(&parent_pid)) {
            parent_node.add_child(child_node);
        }
    }
    
    // Collect all remaining root processes (those not added as children)
    process_map.into_values().collect()
}
