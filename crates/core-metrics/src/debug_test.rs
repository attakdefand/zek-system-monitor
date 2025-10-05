// Test file to debug what data is being collected
use sysinfo::{System, Networks, Disks};

pub fn debug_data_collection() {
    println!("Starting debug data collection...");
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let load_avg = sysinfo::System::load_average();
    
    // Print basic system info
    println!("CPU Count: {}", sys.cpus().len());
    println!("Total Memory: {} KB", sys.total_memory());
    println!("Available Memory: {} KB", sys.available_memory());
    println!("Load Average: {:.2} {:.2} {:.2}", load_avg.one, load_avg.five, load_avg.fifteen);
    
    // Collect network data
    let networks = Networks::new_with_refreshed_list();
    println!("Network Interfaces Count: {}", networks.len());
    for (name, data) in &networks {
        println!("  {}: RX={} bytes, TX={} bytes", 
                 name, 
                 data.total_received(), 
                 data.total_transmitted());
    }
    
    // Collect disk data
    let disks = Disks::new_with_refreshed_list();
    println!("Disks Count: {}", disks.len());
    for disk in &disks {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let usage = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        println!("  Name: '{}', Mount Point: '{}' - {} total, {} available, {} used ({:.1}%)", 
                 disk.name().to_string_lossy(),
                 disk.mount_point().to_string_lossy(),
                 total,
                 available,
                 used,
                 usage);
    }
    
    // Collect process data
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
    
    println!("Top 5 Processes by CPU Usage:");
    for (i, (pid, process)) in processes.iter().take(5).enumerate() {
        println!("  {}: {} (PID: {}, CPU: {}%, Memory: {} KB)", 
                 i + 1,
                 process.name(),
                 pid,
                 process.cpu_usage(),
                 process.memory());
    }
    
    println!("Debug data collection completed.");
}