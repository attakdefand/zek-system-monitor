use core_metrics::collectors::{net::NetworkInfo, snapshot::Snapshot};
use std::time::Duration;

fn main() {
    println!("Debugging network throughput calculation...");
    
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
    for net in &initial_networks {
        println!("  {}: RX={} TX={}", net.interface, net.rx_bytes, net.tx_bytes);
    }
    
    // Wait for a bit to simulate time passing
    std::thread::sleep(Duration::from_secs(2));
    
    // Refresh system data
    sys.refresh_all();
    let networks = sysinfo::Networks::new_with_refreshed_list();
    
    // Create a "previous" snapshot
    let now = chrono::Utc::now();
    let prev_timestamp = now.timestamp_millis() - 2000; // 2 seconds ago
    
    let prev_snapshot = Snapshot {
        ts: prev_timestamp,
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
    
    println!("Previous snapshot timestamp: {}", prev_timestamp);
    println!("Current timestamp: {}", now.timestamp_millis());
    println!("Time delta: {} ms", now.timestamp_millis() - prev_timestamp);
    
    // Calculate network data with throughput
    let current_time = chrono::Utc::now();
    let current_networks: Vec<NetworkInfo> = networks
        .iter()
        .map(|(name, data)| {
            let base_info = NetworkInfo::from_network_data(name, data);
            println!("Processing interface: {}", name);
            
            // Find matching previous network data
            if let Some(prev_net) = prev_snapshot.network.iter().find(|n| n.interface == *name) {
                println!("  Found previous data for {}: RX={} TX={}", name, prev_net.rx_bytes, prev_net.tx_bytes);
                println!("  Current data for {}: RX={} TX={}", name, base_info.rx_bytes, base_info.tx_bytes);
                
                let rx_delta = base_info.rx_bytes.saturating_sub(prev_net.rx_bytes);
                let tx_delta = base_info.tx_bytes.saturating_sub(prev_net.tx_bytes);
                let time_delta_ms = (current_time.timestamp_millis() - prev_snapshot.ts) as f64;
                
                println!("  RX delta: {}, TX delta: {}, Time delta: {} ms", rx_delta, tx_delta, time_delta_ms);
                
                let rx_throughput = (rx_delta as f64) / (time_delta_ms / 1000.0);
                let tx_throughput = (tx_delta as f64) / (time_delta_ms / 1000.0);
                
                println!("  Calculated throughput: RX={:.2} B/s, TX={:.2} B/s", rx_throughput, tx_throughput);
                
                let result = base_info.with_throughput(prev_net, time_delta_ms);
                println!("  Result throughput: RX={:.2} B/s, TX={:.2} B/s", result.rx_throughput, result.tx_throughput);
                result
            } else {
                println!("  No previous data found for {}", name);
                base_info
            }
        })
        .collect();
    
    println!("\nFinal results:");
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