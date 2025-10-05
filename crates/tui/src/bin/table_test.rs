use ratatui::{prelude::*, widgets::*};
use std::io;
use crossterm::{event, execute, terminal};

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

fn create_network_table() -> Table<'static> {
    let header = Row::new(vec!["Interface", "RX Bytes", "TX Bytes", "RX Throughput", "TX Throughput"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    
    let rows: Vec<Row> = vec![
        Row::new(vec![
            "Ethernet".to_string(),
            format_bytes(1784913734),
            format_bytes(502636246),
            format_throughput(4666.666666666666),
            format_throughput(2463.768115942029),
        ]).style(Style::default().fg(Color::White)),
        Row::new(vec![
            "vEthernet (WSL)".to_string(),
            format_bytes(36324),
            format_bytes(28841),
            format_throughput(0.0),
            format_throughput(0.0),
        ]).style(Style::default().fg(Color::White)),
        Row::new(vec![
            "vEthernet (Default Switch)".to_string(),
            format_bytes(39877),
            format_bytes(133636),
            format_throughput(0.0),
            format_throughput(0.0),
        ]).style(Style::default().fg(Color::White)),
    ];
    
    Table::new(
        rows,
        &[
            Constraint::Length(25),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Length(12),
        ]
    )
    .header(header)
    .block(Block::default().title("Network Interfaces (3)").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    let mut out = io::stdout();
    execute!(out, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    let mut term = Terminal::new(backend)?;
    
    loop {
        term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(10),
                ])
                .split(f.area());
            
            let network_table = create_network_table();
            f.render_widget(network_table, chunks[0]);
        })?;
        
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(k) = event::read()? {
                use event::KeyCode::*;
                if matches!(k.code, Char('q') | Esc) {
                    break;
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