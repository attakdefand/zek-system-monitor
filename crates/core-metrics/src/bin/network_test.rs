use core_metrics::collectors::{net::NetworkInfo, snapshot::Snapshot};
use sysinfo::NetworkData;
use std::time::Duration;

fn main() {
    println!("Testing network throughput calculation...");
    
    // Create a mock NetworkData for testing
    let now = chrono::Utc::now();
    let load_avg = sysinfo::LoadAvg { one: 0.0, five: 0.0, fifteen: 0.0 };
    
    // Create system object
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    
    // Get initial network data
    let networks = sysinfo::Networks::new_with_refreshed_list();
    
    let initial_networks: Vec<NetworkInfo> = networks
        .iter()
        .map(|(name, data)| NetworkInfo::from_network_data(name, data))
        .collect();
    
    println!("Initial network data collected for {} interfaces", initial_networks.len());
    
    // Wait for a bit to simulate time passing
    std::thread::sleep(Duration::from_secs(2));
    
    // Refresh system data
    sys.refresh_all();
    let networks = sysinfo::Networks::new_with_refreshed_list();
    
    // Create a "previous" snapshot
    let prev_snapshot = Snapshot {
        ts: now.timestamp_millis(),
        cpu_total_pct: 0.0,
        cpu_per_core: vec![],
        mem_used_bytes: 0,
        mem_total_bytes: 0,
        swap_used_bytes: 0,
        swap_total_bytes: 0,
        load1: 0.0,
        load5: 0.0,
        load15: 0.0,
        network: initial_networks,
        disks: vec![],
        top_processes: vec![],
        sensors: vec![],
        batteries: vec![],
        gpus: vec![],
        connections: vec![],
        process_tree: vec![],
        containers: vec![],
    };
    
    // Calculate network data with throughput
    let current_networks: Vec<NetworkInfo> = networks
        .iter()
        .map(|(name, data)| {
            let base_info = NetworkInfo::from_network_data(name, data);
            // Find matching previous network data
            if let Some(prev_net) = prev_snapshot.network.iter().find(|n| n.interface == *name) {
                let time_delta_ms = (chrono::Utc::now().timestamp_millis() - prev_snapshot.ts) as f64;
                base_info.with_throughput(prev_net, time_delta_ms)
            } else {
                base_info
            }
        })
        .collect();
    
    println!("Current network data with throughput for {} interfaces", current_networks.len());
    
    // Display results
    for net in &current_networks {
        println!(
            "Interface: {} - RX: {} bytes, TX: {} bytes, RX Throughput: {:.2} B/s, TX Throughput: {:.2} B/s",
            net.interface,
            net.rx_bytes,
            net.tx_bytes,
            net.rx_throughput,
            net.tx_throughput
        );
    }
}