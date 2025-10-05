fn main() {
    // Test the correct sysinfo API
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    
    // Print some basic info to understand the API
    println!("Total memory: {}", sys.total_memory());
    println!("Available memory: {}", sys.available_memory());
    println!("CPU count: {}", sys.cpus().len());
    
    // Try to understand the correct API for networks, disks, and processes
    // Based on the error, it seems we need to create separate objects for these
    let networks = sysinfo::Networks::new_with_refreshed_list();
    println!("Networks count: {}", networks.len());
    
    let disks = sysinfo::Disks::new_with_refreshed_list();
    println!("Disks count: {}", disks.len());
    
    // For processes, we still use the system object
    println!("Processes count: {}", sys.processes().len());
}