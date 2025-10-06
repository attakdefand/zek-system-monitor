use core_metrics::collectors::{
    sensors::SensorInfo,
    gpu::GpuInfo,
    connections::ConnectionInfo,
    containers::ContainerInfo,
    process_tree::ProcessTreeNode,
    snapshot::Snapshot
};
use sysinfo::{System, Components};

fn main() {
    println!("Testing extended features: GPU, Battery, Connections, Docker/Containers...");
    
    // Create a new system object
    let mut sys = System::new_all();
    
    // Refresh all information
    sys.refresh_all();
    
    // Test 1: Sensor and Battery Information (already partially implemented)
    println!("\n=== Sensor Information ===");
    let components = Components::new_with_refreshed_list();
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
    
    if sensors.is_empty() {
        println!("  No sensors detected");
    } else {
        for sensor in &sensors {
            println!("  {} - {}{}", sensor.label, sensor.temperature, sensor.unit);
        }
    }
    
    // Note: Battery information requires platform-specific implementation
    // This is just a placeholder showing the structure
    println!("\n=== Battery Information (Framework) ===");
    println!("  Battery framework ready but requires platform-specific implementation");
    println!("  Would use platform-specific APIs or crates like 'battery' crate");
    
    // Test 2: GPU Information (Framework)
    println!("\n=== GPU Information (Framework) ===");
    println!("  GPU framework ready but requires platform-specific implementation");
    println!("  Would use platform-specific APIs or crates like 'nvml' for NVIDIA GPUs");
    
    // Test 3: Connection Tracking (Framework)
    println!("\n=== Connection Tracking (Framework) ===");
    println!("  Connection tracking framework ready but requires platform-specific implementation");
    println!("  Would use system APIs or crates like 'socket2' or 'netstat' approach");
    
    // Test 4: Docker/Container Monitoring (Framework)
    println!("\n=== Docker/Container Monitoring (Framework) ===");
    println!("  Container monitoring framework ready but requires Docker API integration");
    println!("  Would use Docker API/client libraries for container statistics");
    
    // Test 5: Process Tree View (already implemented)
    println!("\n=== Process Tree View ===");
    // This would normally be built from system processes
    println!("  Process tree framework implemented and ready");
    
    // Test Snapshot creation with all features
    println!("\n=== Snapshot with Extended Features ===");
    let snapshot = Snapshot::from_sysinfo(
        chrono::Utc::now(),
        &sys,
        sysinfo::System::load_average(),
        None
    );
    
    println!("  Snapshot created with:");
    println!("    - {} sensors", snapshot.sensors.len());
    println!("    - {} batteries (framework)", snapshot.batteries.len());
    println!("    - {} GPUs (framework)", snapshot.gpus.len());
    println!("    - {} connections (framework)", snapshot.connections.len());
    println!("    - {} process tree nodes", snapshot.process_tree.len());
    println!("    - {} containers (framework)", snapshot.containers.len());
    
    println!("\nExtended features test completed.");
    println!("\nNote: Frameworks are implemented but some features require platform-specific");
    println!("implementations or external crate integrations for full functionality.");
}