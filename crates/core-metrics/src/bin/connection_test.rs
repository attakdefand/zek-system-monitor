use core_metrics::collectors::connections::collect_connections;

fn main() {
    println!("Testing connection tracking...");
    
    let connections = collect_connections();
    
    println!("Found {} connections", connections.len());
    
    for (i, conn) in connections.iter().enumerate() {
        println!(
            "{}. Protocol: {}, Local: {}, Remote: {}, State: {:?}",
            i + 1,
            conn.protocol,
            conn.local_address,
            conn.remote_address,
            conn.state
        );
        
        // Limit output for readability
        if i >= 9 {
            println!("... (showing first 10 connections)");
            break;
        }
    }
    
    if connections.is_empty() {
        println!("No connections found. This could be due to:");
        println!("1. Running on a platform that doesn't support connection tracking yet");
        println!("2. No active network connections");
        println!("3. Insufficient permissions to read connection information");
    }
}