use crossterm::{event, terminal, execute};
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    // Enable raw mode
    terminal::enable_raw_mode()?;
    
    // Enter alternate screen
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    
    println!("Key Event Test - Press keys to see their codes (Press 'q' to quit)");
    println!("================================================================");
    
    loop {
        // Poll for events
        if event::poll(std::time::Duration::from_millis(100))? {
            // Read the event
            if let event::Event::Key(key) = event::read()? {
                // Print key information
                println!("Key pressed: {:?} | Code: {:?} | Modifiers: {:?}", 
                         key.kind, key.code, key.modifiers);
                
                // Flush stdout to see output immediately
                io::stdout().flush()?;
                
                // Exit on 'q'
                if let event::KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }
    
    // Restore terminal
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    
    Ok(())
}