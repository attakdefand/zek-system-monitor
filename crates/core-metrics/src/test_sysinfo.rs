use sysinfo::System;

pub fn test_sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Print available methods
    println!("Available networks: {}", sys.networks().len());
    println!("Available disks: {}", sys.disks().len());
    println!("Available processes: {}", sys.processes().len());
}