# Zek System Monitor - Usage Guide

## Starting the Application

To start the Zek system monitor, run the following command from the project root directory:

```bash
cargo run
```

This will launch the terminal-based user interface with real-time system metrics.

## Navigation Controls

### Table Switching

- **Tab Key**: Cycle forward through tables in the following order:
  1. Network Interfaces
  2. Disk Usage
  3. System Sensors
  4. GPU Usage
  5. Containers
  6. Top Processes

- **Shift+Tab**: Cycle backward through tables in reverse order

### Item Navigation Within Tables

- **Down Arrow**: Move selection down to the next item in the currently selected table
- **Up Arrow**: Move selection up to the previous item in the currently selected table
- Navigation automatically wraps around when reaching the end/beginning of a table

### Exiting the Application

- **Q Key**: Quit the application
- **Esc Key**: Quit the application

## Step-by-Step Navigation Example

1. Start the application: `cargo run`
2. The Network table will be selected by default
3. Press **Tab** to switch to the Disk table
4. Press **Down Arrow** to select the first disk entry
5. Press **Down Arrow** again to move to the second disk entry
6. Press **Tab** to switch to the Sensors table
7. Continue navigating with **Tab** and arrow keys
8. Press **Q** or **Esc** to quit

## Visual Feedback

- The currently selected table has a highlighted border
- The currently selected item in a table is highlighted with a reversed color scheme
- A ">>" symbol appears next to the selected item
- The footer displays the current table name and navigation instructions

## System Requirements

- Terminal size: Minimum 80x24 characters recommended
- Rust 1.81 or later for building from source
- Supported platforms: Windows, Linux, macOS

## Configuration

The application uses the configuration file located at `configs/zek.toml`. You can modify refresh intervals and other settings in this file.