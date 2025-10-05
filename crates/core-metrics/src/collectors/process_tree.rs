use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTreeNode {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage_bytes: u64,
    pub children: Vec<ProcessTreeNode>,
    pub parent_pid: Option<u32>,
}

impl ProcessTreeNode {
    pub fn new(
        pid: u32,
        name: String,
        cpu_usage: f32,
        memory_usage_bytes: u64,
        parent_pid: Option<u32>,
    ) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            memory_usage_bytes,
            children: Vec::new(),
            parent_pid,
        }
    }
    
    pub fn add_child(&mut self, child: ProcessTreeNode) {
        self.children.push(child);
    }
}