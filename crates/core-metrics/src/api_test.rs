// Test file to check the correct sysinfo API methods
use sysinfo::{System, Networks, Disks};

pub fn test_api() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Check the correct way to access networks
    println!("Networks:");
    for (interface_name, network_data) in sys.networks() {
        println!("  {}: {} bytes received, {} bytes transmitted", 
                 interface_name, 
                 network_data.total_received(), 
                 network_data.total_transmitted());
    }
    
    // Check the correct way to access disks
    println!("Disks:");
    for disk in sys.disks() {
        println!("  {}: {} total, {} available", 
                 disk.name().to_string_lossy(),
                 disk.total_space(),
                 disk.available_space());
    }
    
    // Check the correct way to access processes
    println!("Processes:");
    for (pid, process) in sys.processes() {
        println!("  {}: {} (CPU: {}%)", 
                 pid, 
                 process.name().to_string_lossy(),
                 process.cpu_usage());
    }
}