use sysinfo::{System, Networks, Disks, Components};

fn main() {
    println!("Testing sysinfo features...");
    
    // Create a new system object
    let mut sys = System::new_all();
    
    // Refresh all information
    sys.refresh_all();
    
    // Test CPU information
    println!("CPU Information:");
    for cpu in sys.cpus() {
        println!("  {} - {}%", cpu.name(), cpu.cpu_usage());
    }
    
    // Test memory information
    println!("Memory Information:");
    println!("  Total Memory: {} KB", sys.total_memory());
    println!("  Available Memory: {} KB", sys.available_memory());
    println!("  Total Swap: {} KB", sys.total_swap());
    println!("  Free Swap: {} KB", sys.free_swap());
    
    // Test network information
    println!("Network Information:");
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!("  {}: {} bytes received, {} bytes transmitted", 
                 interface_name, data.total_received(), data.total_transmitted());
    }
    
    // Test disk information
    println!("Disk Information:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("  {:?} - Total: {} KB, Available: {} KB", 
                 disk.mount_point(), disk.total_space() / 1024, disk.available_space() / 1024);
    }
    
    // Test components (sensors)
    println!("Component Information:");
    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("  {} - {}°C", component.label(), component.temperature());
    }
    
    // Test processes
    println!("Process Information:");
    for (pid, process) in sys.processes() {
        println!("  {} - {} (CPU: {}%)", pid, process.name(), process.cpu_usage());
    }
    
    println!("Feature test completed.");
}