use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub state: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

impl ConnectionInfo {
    pub fn new(
        protocol: String,
        local_address: String,
        remote_address: String,
        state: String,
        pid: Option<u32>,
        process_name: Option<String>,
    ) -> Self {
        Self {
            protocol,
            local_address,
            remote_address,
            state,
            pid,
            process_name,
        }
    }
}

#[cfg(target_os = "windows")]
pub fn collect_connections() -> Vec<ConnectionInfo> {
    // Windows implementation using sysinfo or Windows APIs
    // For now, return empty vector as sysinfo doesn't provide connection info
    Vec::new()
}

#[cfg(target_os = "linux")]
pub fn collect_connections() -> Vec<ConnectionInfo> {
    // Linux implementation reading from /proc/net/
    use std::fs;
    
    let mut connections = Vec::new();
    
    // Try to read TCP connections
    if let Ok(tcp_data) = fs::read_to_string("/proc/net/tcp") {
        connections.extend(parse_proc_net_file(&tcp_data, "TCP"));
    }
    
    // Try to read UDP connections
    if let Ok(udp_data) = fs::read_to_string("/proc/net/udp") {
        connections.extend(parse_proc_net_file(&udp_data, "UDP"));
    }
    
    connections
}

#[cfg(target_os = "macos")]
pub fn collect_connections() -> Vec<ConnectionInfo> {
    // macOS implementation using system calls or lsof
    // For now, return empty vector
    Vec::new()
}

#[cfg(target_os = "linux")]
fn parse_proc_net_file(data: &str, protocol: &str) -> Vec<ConnectionInfo> {
    let mut connections = Vec::new();
    
    // Skip the header line
    for line in data.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 {
            continue;
        }
        
        // Parse local and remote addresses
        let local_addr = parse_address(fields[1]);
        let remote_addr = parse_address(fields[2]);
        let state = parse_tcp_state(fields[3]);
        
        connections.push(ConnectionInfo::new(
            protocol.to_string(),
            local_addr,
            remote_addr,
            state,
            None, // PID not available in this file
            None, // Process name not available in this file
        ));
    }
    
    connections
}

#[cfg(target_os = "linux")]
fn parse_address(addr_str: &str) -> String {
    let parts: Vec<&str> = addr_str.split(':').collect();
    if parts.len() != 2 {
        return addr_str.to_string();
    }
    
    let ip_hex = parts[0];
    let port_hex = parts[1];
    
    // Parse IP address (assuming IPv4 for simplicity)
    if ip_hex.len() == 8 {
        let ip_bytes = (0..4)
            .map(|i| u8::from_str_radix(&ip_hex[i*2..i*2+2], 16).unwrap_or(0))
            .collect::<Vec<u8>>();
        
        let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]));
        let port = u16::from_str_radix(port_hex, 16).unwrap_or(0);
        
        format!("{}:{}", ip, port)
    } else {
        addr_str.to_string()
    }
}

#[cfg(target_os = "linux")]
fn parse_tcp_state(state_hex: &str) -> String {
    match state_hex {
        "01" => "ESTABLISHED",
        "02" => "SYN_SENT",
        "03" => "SYN_RECV",
        "04" => "FIN_WAIT1",
        "05" => "FIN_WAIT2",
        "06" => "TIME_WAIT",
        "07" => "CLOSE",
        "08" => "CLOSE_WAIT",
        "09" => "LAST_ACK",
        "0A" => "LISTEN",
        "0B" => "CLOSING",
        _ => "UNKNOWN",
    }.to_string()
}

// Fallback implementation for other platforms
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn collect_connections() -> Vec<ConnectionInfo> {
    Vec::new()
}