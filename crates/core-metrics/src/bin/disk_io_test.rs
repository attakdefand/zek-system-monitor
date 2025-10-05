use sysinfo::Disks;

fn main() {
    println!("Testing disk I/O capabilities...");
    
    let disks = Disks::new_with_refreshed_list();
    
    for disk in &disks {
        println!("Disk: {:?}", disk.name());
        println!("  Mount point: {:?}", disk.mount_point());
        println!("  Total space: {}", disk.total_space());
        println!("  Available space: {}", disk.available_space());
        println!("  File system: {:?}", disk.file_system());
        println!("  Is removable: {}", disk.is_removable());
        // Check if there are any I/O related methods
        // Note: In sysinfo 0.30, disk I/O statistics are not directly available
        // We would need to use other methods or external crates for this
    }
    
    println!("Disk I/O statistics are not directly available in sysinfo crate version 0.30.");
    println!("For disk I/O monitoring, we would need to use platform-specific APIs or other crates.");
}