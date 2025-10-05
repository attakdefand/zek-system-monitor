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