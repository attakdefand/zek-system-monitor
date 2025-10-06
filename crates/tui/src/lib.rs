use anyhow::Result; 
use core_metrics::{Supervisor, collectors::{snapshot::Snapshot, net::NetworkInfo, disk::DiskInfo, process::ProcessInfo, sensors::SensorInfo, gpu::GpuInfo, containers::ContainerInfo}};
use crossterm::{event, execute, terminal}; 
use ratatui::{prelude::*, widgets::*}; 
use std::{io, time::Duration};
use tracing::debug;

// State struct to track which table is selected and the selection index
#[derive(Debug, Clone)]
struct TuiState {
    selected_table: usize, // 0 = Network, 1 = Disk, 2 = Process, 3 = Sensors, 4 = GPU, 5 = Containers
    table_selections: [usize; 6], // Selection index for each table
    table_states: [TableState; 6], // State for each table widget
}

impl TuiState {
    fn new() -> Self {
        Self {
            selected_table: 0,
            table_selections: [0; 6],
            table_states: Default::default(), // This will initialize all TableStates to default
        }
    }
    
    fn next_table(&mut self) {
        self.selected_table = (self.selected_table + 1) % 6;
    }
    
    fn previous_table(&mut self) {
        self.selected_table = if self.selected_table == 0 {
            5
        } else {
            self.selected_table - 1
        };
    }
    
    fn next_item(&mut self, max_items: usize) {
        if max_items > 0 {
            self.table_selections[self.selected_table] = 
                (self.table_selections[self.selected_table] + 1) % max_items;
            self.table_states[self.selected_table].select(Some(self.table_selections[self.selected_table]));
        }
    }
    
    fn previous_item(&mut self, max_items: usize) {
        if max_items > 0 {
            self.table_selections[self.selected_table] = 
                if self.table_selections[self.selected_table] == 0 {
                    max_items - 1
                } else {
                    self.table_selections[self.selected_table] - 1
                };
            self.table_states[self.selected_table].select(Some(self.table_selections[self.selected_table]));
        }
    }
    
    fn current_selection(&self) -> usize {
        self.table_selections[self.selected_table]
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

fn format_throughput(bytes_per_sec: f64) -> String {
    const UNITS: &[&str] = &["B/s", "KiB/s", "MiB/s", "GiB/s"];
    let mut size = bytes_per_sec;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

fn create_network_table(networks: &[NetworkInfo], _selected: bool, _selection_index: usize) -> Table<'_> {
    // Debug: Print network data
    debug!("Creating network table with {} interfaces", networks.len());
    for net in networks {
        debug!("Network interface: {} - RX: {}, TX: {}, RX Throughput: {}, TX Throughput: {}", 
                  net.interface, net.rx_bytes, net.tx_bytes, net.rx_throughput, net.tx_throughput);
    }
    
    // Simple test to see if data is being passed
    if networks.is_empty() {
        let rows = vec![Row::new(vec!["No network data available"])];
        return Table::new(rows, [Constraint::Percentage(100)])
            .block(Block::default().title("Network Interfaces (0)").borders(Borders::ALL));
    }
    
    let header = Row::new(vec!["Interface", "RX Bytes", "TX Bytes", "RX Throughput", "TX Throughput"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = networks.iter().map(|net| {
        let row_data = vec![
            net.interface.clone(),
            format_bytes(net.rx_bytes),
            format_bytes(net.tx_bytes),
            format_throughput(net.rx_throughput),
            format_throughput(net.tx_throughput),
        ];
        // Debug: Print row data
        debug!("Row data: {:?}", row_data);
        
        let style = Style::default().fg(Color::White);
        Row::new(row_data).style(style)
    }).collect();
    
    // Create a simple table with consistent length constraints
    let table = Table::new(
        rows,
        [
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(12),
        ]
    )
    .header(header)
    .block(Block::default().title(format!("Network Interfaces ({})", networks.len())).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
    
    // Debug: Print table info
    debug!("Network table created");
    table
}

fn create_disk_table(disks: &[DiskInfo], _selected: bool, _selection_index: usize) -> Table<'_> {
    // Debug: Print disk data
    debug!("Creating disk table with {} disks", disks.len());
    for disk in disks {
        debug!("Disk: {} - Total: {}, Used: {}, Available: {}, Usage: {}", 
                  disk.mount_point, disk.total_space, disk.used_space, disk.available_space, disk.usage_percent);
    }
    
    if disks.is_empty() {
        let rows = vec![Row::new(vec!["No disk data available"])];
        return Table::new(rows, [Constraint::Percentage(100)])
            .block(Block::default().title("Disk Usage (0)").borders(Borders::ALL));
    }
    
    let header = Row::new(vec!["Mount Point", "Total", "Used", "Available", "Usage"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = disks.iter().map(|disk| {
        let usage_color = if disk.usage_percent < 70.0 {
            Color::Green
        } else if disk.usage_percent < 85.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        
        let row_data = vec![
            disk.mount_point.clone(),
            format_bytes(disk.total_space),
            format_bytes(disk.used_space),
            format_bytes(disk.available_space),
            format!("{:.1}%", disk.usage_percent),
        ];
        // Debug: Print row data
        debug!("Disk row data: {:?}", row_data);
        
        Row::new(row_data).style(Style::default().fg(usage_color))
    }).collect();
    
    // Create a simple table with consistent percentage constraints
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ]
    )
    .header(header)
    .block(Block::default().title(format!("Disk Usage ({})", disks.len())).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
    
    // Debug: Print table info
    debug!("Disk table created");
    table
}

fn create_process_table(processes: &[ProcessInfo], _selected: bool, _selection_index: usize) -> Table<'_> {
    // Debug: Print process data
    debug!("Creating process table with {} processes", processes.len());
    for proc in processes {
        debug!("Process: {} - PID: {}, Name: {}, CPU: {}, Memory: {}", 
                  proc.name, proc.pid, proc.name, proc.cpu_usage, proc.memory);
    }
    
    if processes.is_empty() {
        let rows = vec![Row::new(vec!["No process data available"])];
        return Table::new(rows, [Constraint::Percentage(100)])
            .block(Block::default().title("Top Processes (0)").borders(Borders::ALL));
    }
    
    let header = Row::new(vec!["PID", "Name", "CPU%", "Memory"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = processes.iter().map(|proc| {
        let cpu_color = if proc.cpu_usage < 30.0 {
            Color::Green
        } else if proc.cpu_usage < 70.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        
        let row_data = vec![
            format!("{}", proc.pid),
            proc.name.clone(),
            format!("{:.1}", proc.cpu_usage),
            format_bytes(proc.memory),
        ];
        // Debug: Print row data
        debug!("Process row data: {:?}", row_data);
        
        Row::new(row_data).style(Style::default().fg(cpu_color))
    }).collect();
    
    // Create a simple table with consistent percentage constraints
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(15),
            Constraint::Percentage(40),
            Constraint::Percentage(15),
            Constraint::Percentage(30),
        ]
    )
    .header(header)
    .block(Block::default().title(format!("Top Processes ({})", processes.len())).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
    
    // Debug: Print table info
    debug!("Process table created");
    table
}

fn create_containers_table(containers: &[ContainerInfo], _selected: bool, _selection_index: usize) -> Table<'_> {
    if containers.is_empty() {
        let rows = vec![Row::new(vec!["No container data available"])];
        return Table::new(rows, [Constraint::Percentage(100)])
            .block(Block::default().title("Containers (0)").borders(Borders::ALL));
    }
    
    let header = Row::new(vec!["Name", "State", "CPU%", "Memory", "Network"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = containers.iter().map(|container| {
        let state_color = match container.state {
            core_metrics::collectors::containers::ContainerState::Running => Color::Green,
            core_metrics::collectors::containers::ContainerState::Paused => Color::Yellow,
            core_metrics::collectors::containers::ContainerState::Stopped => Color::Red,
            _ => Color::White,
        };
        
        let row_data = vec![
            container.name.clone(),
            format!("{:?}", container.state),
            format!("{:.1}", container.cpu_usage_percent),
            format_bytes(container.memory_usage_bytes),
            format!("RX: {} TX: {}", format_bytes(container.network_rx_bytes), format_bytes(container.network_tx_bytes)),
        ];
        
        Row::new(row_data).style(Style::default().fg(state_color))
    }).collect();
    
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]
    )
    .header(header)
    .block(Block::default().title(format!("Containers ({})", containers.len())).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
    
    table
}

fn create_sensors_table(sensors: &[SensorInfo], _selected: bool, _selection_index: usize) -> Table<'_> {
    if sensors.is_empty() {
        let rows = vec![Row::new(vec!["No sensor data available"])];
        return Table::new(rows, [Constraint::Percentage(100)])
            .block(Block::default().title("System Sensors (0)").borders(Borders::ALL));
    }
    
    let header = Row::new(vec!["Component", "Label", "Temperature"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = sensors.iter().map(|sensor| {
        let temp_color = if sensor.temperature < 50.0 {
            Color::Green
        } else if sensor.temperature < 70.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        
        let row_data = vec![
            sensor.component.clone(),
            sensor.label.clone(),
            format!("{:.1}°C", sensor.temperature),
        ];
        
        Row::new(row_data).style(Style::default().fg(temp_color))
    }).collect();
    
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ]
    )
    .header(header)
    .block(Block::default().title(format!("System Sensors ({})", sensors.len())).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
    
    table
}

fn create_gpu_table(gpus: &[GpuInfo], _selected: bool, _selection_index: usize) -> Table<'_> {
    if gpus.is_empty() {
        let rows = vec![Row::new(vec!["No GPU data available"])];
        return Table::new(rows, [Constraint::Percentage(100)])
            .block(Block::default().title("GPU Usage (0)").borders(Borders::ALL));
    }
    
    let header = Row::new(vec!["GPU", "Usage", "Memory", "Temperature", "Fan"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = gpus.iter().map(|gpu| {
        let usage_color = if gpu.usage_percent < 50.0 {
            Color::Green
        } else if gpu.usage_percent < 80.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        
        let row_data = vec![
            gpu.name.clone(),
            format!("{:.1}%", gpu.usage_percent),
            format!("{}/{} MiB", gpu.memory_used_bytes / 1024 / 1024, gpu.memory_total_bytes / 1024 / 1024),
            format!("{:.1}°C", gpu.temperature),
            format!("{:.1}%", gpu.fan_speed_percent),
        ];
        
        Row::new(row_data).style(Style::default().fg(usage_color))
    }).collect();
    
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
        ]
    )
    .header(header)
    .block(Block::default().title(format!("GPU Usage ({})", gpus.len())).borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");
    
    table
}

pub async fn run_tui(sup: Supervisor) -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut out = io::stdout();
    execute!(out, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let mut term = Terminal::new(backend)?;
    
    let mut last: Option<Snapshot> = None;
    let mut state = TuiState::new();
    
    loop {
        while let Ok(s) = sup.subscribe().try_recv() {
            last = Some(s);
        }
        
        term.draw(|f| {
            // Get terminal size to adjust layout
            let terminal_size = f.area();
            let height = terminal_size.height;
            
            // Adjust layout based on terminal height
            let constraints = if height >= 30 {
                // Use full layout for larger terminals
                vec![
                    Constraint::Length(3),    // Title
                    Constraint::Length(5),    // CPU + Memory
                    Constraint::Length(5),    // Load + Network Summary
                    Constraint::Length(8),   // Per-core CPU usage
                    Constraint::Length(12),  // Network Data
                    Constraint::Length(12),  // Disk Data
                    Constraint::Length(12),  // Sensors
                    Constraint::Length(12),  // GPU
                    Constraint::Length(12),  // Containers
                    Constraint::Length(18),  // Process Data
                    Constraint::Length(2),   // Footer
                ]
            } else if height >= 20 {
                // Use compact layout for medium terminals
                vec![
                    Constraint::Length(3),    // Title
                    Constraint::Length(5),    // CPU + Memory
                    Constraint::Length(5),    // Load + Network Summary
                    Constraint::Length(6),   // Per-core CPU usage
                    Constraint::Length(10),  // Network Data
                    Constraint::Length(10),  // Disk Data
                    Constraint::Length(10),  // Sensors
                    Constraint::Length(10),  // GPU
                    Constraint::Length(10),  // Containers
                    Constraint::Length(15),  // Process Data
                    Constraint::Length(2),   // Footer
                ]
            } else {
                // Use minimal layout for small terminals
                vec![
                    Constraint::Length(3),    // Title
                    Constraint::Length(3),    // CPU + Memory
                    Constraint::Length(3),    // Load + Network Summary
                    Constraint::Length(5),   // Per-core CPU usage
                    Constraint::Length(8),   // Network + Disk Data (combined)
                    Constraint::Length(8),   // Sensors + GPU (combined)
                    Constraint::Length(8),   // Containers + Process Data (combined)
                    Constraint::Length(2),   // Footer
                ]
            };
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints)
                .split(f.area());
            
            // Title
            let title = Paragraph::new("ZEK SYSTEM MONITOR")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            f.render_widget(title, chunks[0]);
            
            if let Some(s) = &last {
                // Debug: Print snapshot info
                debug!("Rendering snapshot with {} network interfaces, {} disks, {} processes", 
                          s.network.len(), s.disks.len(), s.top_processes.len());
                
                // For small terminals, we need to combine sections
                if height < 20 {
                    // CPU and Memory row
                    let cpu_mem_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[1]);
                    
                    // CPU Gauge
                    let cpu_ratio = (s.cpu_total_pct / 100.0).clamp(0.0, 1.0);
                    let cpu_color = if cpu_ratio < 0.5 { Color::Green } else if cpu_ratio < 0.8 { Color::Yellow } else { Color::Red };
                    let cpu_gauge = Gauge::default()
                        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                        .gauge_style(Style::default().fg(cpu_color))
                        .percent(s.cpu_total_pct as u16)
                        .label(format!("{:.1}%", s.cpu_total_pct));
                    f.render_widget(cpu_gauge, cpu_mem_chunks[0]);
                    
                    // Memory Gauge (including swap)
                    let mem_used = s.mem_used_bytes as f64;
                    let mem_total = s.mem_total_bytes as f64;
                    let swap_used = s.swap_used_bytes as f64;
                    let swap_total = s.swap_total_bytes as f64;
                    
                    let mem_ratio = if mem_total > 0.0 { (mem_used / mem_total).clamp(0.0, 1.0) } else { 0.0 };
                    
                    let mem_color = if mem_ratio < 0.7 { Color::Green } else if mem_ratio < 0.85 { Color::Yellow } else { Color::Red };
                    
                    // Create a multi-line label showing both memory and swap
                    let mem_label = format!("{:.1}/{:.1} GiB\nSwap: {:.1}/{:.1} GiB", 
                        mem_used/1.0737e9, mem_total/1.0737e9,
                        swap_used/1.0737e9, swap_total/1.0737e9);
                    
                    let mem_gauge = Gauge::default()
                        .block(Block::default().title("Memory & Swap").borders(Borders::ALL))
                        .gauge_style(Style::default().fg(mem_color))
                        .ratio(mem_ratio)
                        .label(mem_label);
                    f.render_widget(mem_gauge, cpu_mem_chunks[1]);
                    
                    // Load and Network Summary row
                    let load_net_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[2]);
                    
                    // Load Average
                    let load_text = format!("Load: {:.2} {:.2} {:.2}", s.load1, s.load5, s.load15);
                    let load_widget = Paragraph::new(load_text)
                        .block(Block::default().title("System Load").borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(load_widget, load_net_chunks[0]);
                    
                    // Network Summary
                    let total_rx: u64 = s.network.iter().map(|n| n.rx_bytes).sum();
                    let total_tx: u64 = s.network.iter().map(|n| n.tx_bytes).sum();
                    let total_rx_throughput: f64 = s.network.iter().map(|n| n.rx_throughput).sum();
                    let total_tx_throughput: f64 = s.network.iter().map(|n| n.tx_throughput).sum();
                    let net_summary = format!("RX {} ({})\nTX {} ({})", 
                        format_bytes(total_rx), format_throughput(total_rx_throughput),
                        format_bytes(total_tx), format_throughput(total_tx_throughput));
                    let net_widget = Paragraph::new(net_summary)
                        .block(Block::default().title("Network").borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(net_widget, load_net_chunks[1]);
                    
                    // Per-core CPU Usage
                    let mut cpu_text = String::new();
                    for (i, cpu_usage) in s.cpu_per_core.iter().enumerate() {
                        // Show every 4th core to avoid overcrowding
                        if i % 4 == 0 || i == s.cpu_per_core.len() - 1 {
                            cpu_text.push_str(&format!("CPU{}: {:.1}%  ", i, cpu_usage));
                        }
                        // Add newline every few CPUs
                        if (i + 1) % 4 == 0 {
                            cpu_text.push('\n');
                        }
                    }
                    if cpu_text.is_empty() {
                        cpu_text = "No CPU data".to_string();
                    }
                    let cpu_widget = Paragraph::new(cpu_text)
                        .block(Block::default().title(format!("Per-Core CPU ({})", s.cpu_per_core.len())).borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(cpu_widget, chunks[3]);
                    
                    // Combined Network and Disk Data
                    let net_disk_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[4]);
                    
                    // Network Data
                    let network_table = create_network_table(&s.network, state.selected_table == 0, state.table_selections[0]);
                    f.render_stateful_widget(network_table, net_disk_chunks[0], &mut state.table_states[0]);
                    
                    // Disk Data
                    let disk_table = create_disk_table(&s.disks, state.selected_table == 1, state.table_selections[1]);
                    f.render_stateful_widget(disk_table, net_disk_chunks[1], &mut state.table_states[1]);
                    
                    // Combined Sensors and GPU
                    let sensor_gpu_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[5]);
                    
                    // Sensors Data
                    let sensors_table = create_sensors_table(&s.sensors, state.selected_table == 2, state.table_selections[2]);
                    f.render_stateful_widget(sensors_table, sensor_gpu_chunks[0], &mut state.table_states[2]);
                    
                    // GPU Data
                    let gpu_table = create_gpu_table(&s.gpus, state.selected_table == 3, state.table_selections[3]);
                    f.render_stateful_widget(gpu_table, sensor_gpu_chunks[1], &mut state.table_states[3]);
                    
                    // Combined Containers and Process Data
                    let cont_proc_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[6]);
                    
                    // Containers Data
                    let containers_table = create_containers_table(&s.containers, state.selected_table == 4, state.table_selections[4]);
                    f.render_stateful_widget(containers_table, cont_proc_chunks[0], &mut state.table_states[4]);
                    
                    // Process Data
                    let process_table = create_process_table(&s.top_processes, state.selected_table == 5, state.table_selections[5]);
                    f.render_stateful_widget(process_table, cont_proc_chunks[1], &mut state.table_states[5]);
                } else {
                    // Use full layout for larger terminals
                    // CPU and Memory row
                    let cpu_mem_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[1]);
                    
                    // CPU Gauge
                    let cpu_ratio = (s.cpu_total_pct / 100.0).clamp(0.0, 1.0);
                    let cpu_color = if cpu_ratio < 0.5 { Color::Green } else if cpu_ratio < 0.8 { Color::Yellow } else { Color::Red };
                    let cpu_gauge = Gauge::default()
                        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                        .gauge_style(Style::default().fg(cpu_color))
                        .percent(s.cpu_total_pct as u16)
                        .label(format!("{:.1}%", s.cpu_total_pct));
                    f.render_widget(cpu_gauge, cpu_mem_chunks[0]);
                    
                    // Memory Gauge (including swap)
                    let mem_used = s.mem_used_bytes as f64;
                    let mem_total = s.mem_total_bytes as f64;
                    let swap_used = s.swap_used_bytes as f64;
                    let swap_total = s.swap_total_bytes as f64;
                    
                    let mem_ratio = if mem_total > 0.0 { (mem_used / mem_total).clamp(0.0, 1.0) } else { 0.0 };
                    
                    let mem_color = if mem_ratio < 0.7 { Color::Green } else if mem_ratio < 0.85 { Color::Yellow } else { Color::Red };
                    
                    // Create a multi-line label showing both memory and swap
                    let mem_label = format!("{:.1}/{:.1} GiB\nSwap: {:.1}/{:.1} GiB", 
                        mem_used/1.0737e9, mem_total/1.0737e9,
                        swap_used/1.0737e9, swap_total/1.0737e9);
                    
                    let mem_gauge = Gauge::default()
                        .block(Block::default().title("Memory & Swap").borders(Borders::ALL))
                        .gauge_style(Style::default().fg(mem_color))
                        .ratio(mem_ratio)
                        .label(mem_label);
                    f.render_widget(mem_gauge, cpu_mem_chunks[1]);
                    
                    // Load and Network Summary row
                    let load_net_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[2]);
                    
                    // Load Average
                    let load_text = format!("Load Average: {:.2} {:.2} {:.2}", s.load1, s.load5, s.load15);
                    let load_widget = Paragraph::new(load_text)
                        .block(Block::default().title("System Load").borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(load_widget, load_net_chunks[0]);
                    
                    // Network Summary
                    let total_rx: u64 = s.network.iter().map(|n| n.rx_bytes).sum();
                    let total_tx: u64 = s.network.iter().map(|n| n.tx_bytes).sum();
                    let total_rx_throughput: f64 = s.network.iter().map(|n| n.rx_throughput).sum();
                    let total_tx_throughput: f64 = s.network.iter().map(|n| n.tx_throughput).sum();
                    let net_summary = format!("Network: RX {} ({}) TX {} ({})", 
                        format_bytes(total_rx), format_throughput(total_rx_throughput),
                        format_bytes(total_tx), format_throughput(total_tx_throughput));
                    let net_widget = Paragraph::new(net_summary)
                        .block(Block::default().title("Network Summary").borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(net_widget, load_net_chunks[1]);
                    
                    // Per-core CPU Usage
                    let mut cpu_text = String::new();
                    for (i, cpu_usage) in s.cpu_per_core.iter().enumerate() {
                        // Show every 4th core to avoid overcrowding
                        if i % 4 == 0 || i == s.cpu_per_core.len() - 1 {
                            cpu_text.push_str(&format!("CPU{}: {:.1}%  ", i, cpu_usage));
                        }
                        // Add newline every few CPUs
                        if (i + 1) % 8 == 0 {
                            cpu_text.push('\n');
                        }
                    }
                    if cpu_text.is_empty() {
                        cpu_text = "No CPU data available".to_string();
                    }
                    let cpu_widget = Paragraph::new(cpu_text)
                        .block(Block::default().title(format!("Per-Core CPU Usage ({} cores)", s.cpu_per_core.len())).borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(cpu_widget, chunks[3]);
                    
                    // Network Data - using proper table widget
                    let network_table = create_network_table(&s.network, state.selected_table == 0, state.table_selections[0]);
                    f.render_stateful_widget(network_table, chunks[4], &mut state.table_states[0]);
                    
                    // Disk Data - using proper table widget
                    let disk_table = create_disk_table(&s.disks, state.selected_table == 1, state.table_selections[1]);
                    f.render_stateful_widget(disk_table, chunks[5], &mut state.table_states[1]);
                    
                    // Sensors Data
                    let sensors_table = create_sensors_table(&s.sensors, state.selected_table == 2, state.table_selections[2]);
                    f.render_stateful_widget(sensors_table, chunks[6], &mut state.table_states[2]);
                    
                    // GPU Data
                    let gpu_table = create_gpu_table(&s.gpus, state.selected_table == 3, state.table_selections[3]);
                    f.render_stateful_widget(gpu_table, chunks[7], &mut state.table_states[3]);
                    
                    // Containers Data
                    let containers_table = create_containers_table(&s.containers, state.selected_table == 4, state.table_selections[4]);
                    f.render_stateful_widget(containers_table, chunks[8], &mut state.table_states[4]);
                    
                    // Process Data - using proper table widget
                    let process_table = create_process_table(&s.top_processes, state.selected_table == 5, state.table_selections[5]);
                    f.render_stateful_widget(process_table, chunks[9], &mut state.table_states[5]);
                }
            } else {
                let loading = Paragraph::new("Initializing system metrics...")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Yellow));
                f.render_widget(loading, chunks[1]);
            }
            
            // Footer with current selection info
            let table_names = ["Network", "Disk", "Sensors", "GPU", "Containers", "Processes"];
            let footer_text = format!("Press 'q' or ESC to quit | Tab: Switch tables | ↑/↓: Navigate | Selected: {}", 
                                     table_names[state.selected_table]);
            let footer = Paragraph::new(footer_text)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));
            f.render_widget(footer, *chunks.last().unwrap());
        })?;
        
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(k) = event::read()? {
                use event::KeyCode::*;
                match k.code {
                    Char('q') | Esc => break,
                    Tab => state.next_table(),
                    BackTab => state.previous_table(),
                    Down => {
                        if let Some(s) = &last {
                            let max_items = match state.selected_table {
                                0 => s.network.len(),
                                1 => s.disks.len(),
                                2 => s.sensors.len(),
                                3 => s.gpus.len(),
                                4 => s.containers.len(),
                                5 => s.top_processes.len(),
                                _ => 0,
                            };
                            state.next_item(max_items);
                        }
                    },
                    Up => {
                        if let Some(s) = &last {
                            let max_items = match state.selected_table {
                                0 => s.network.len(),
                                1 => s.disks.len(),
                                2 => s.sensors.len(),
                                3 => s.gpus.len(),
                                4 => s.containers.len(),
                                5 => s.top_processes.len(),
                                _ => 0,
                            };
                            state.previous_item(max_items);
                        }
                    },
                    _ => {}
                }
            }
        }

    }
    
    terminal::disable_raw_mode()?;
    execute!(
        term.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    term.show_cursor()?;
    Ok(())
}