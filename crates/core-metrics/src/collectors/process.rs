use serde::{Serialize, Deserialize};
use sysinfo::Process;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

impl ProcessInfo {
    pub fn from_process(pid: u32, process: &Process) -> Self {
        Self {
            pid,
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            status: format!("{:?}", process.status()),
        }
    }
}