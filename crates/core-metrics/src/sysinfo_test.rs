// This is a test file to check what sysinfo methods are available
use sysinfo::{System, Networks, Disks};

pub fn test_sysinfo_api() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Check what methods are available
    println!("System info:");
    println!("  Total memory: {}", sys.total_memory());
    println!("  Available memory: {}", sys.available_memory());
    println!("  CPU count: {}", sys.cpus().len());
    
    // For networks, disks, and processes, we need to check the correct API
    println!("  Networks count: {}", sys.networks().len());
    println!("  Disks count: {}", sys.disks().len());
    println!("  Processes count: {}", sys.processes().len());
}