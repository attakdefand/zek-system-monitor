# Zek - System Monitoring Tool

Zek is a comprehensive system monitoring tool built in Rust, inspired by tools like Glances but with a modern microservices architecture. It provides real-time system metrics visualization in the terminal with a single command.

## Features

- Real-time CPU, memory, and system load monitoring
- Network interface statistics with throughput
- Disk usage monitoring
- Process monitoring (top processes)
- Per-core CPU usage
- Swap memory monitoring
- Temperature and sensor monitoring
- GPU utilization monitoring (framework ready)
- Battery status monitoring (framework ready)
- File system monitoring
- Connection tracking (framework ready)
- Process tree view (framework ready)
- Docker/container monitoring (framework ready)
- Terminal-based user interface with visual gauges
- Microservices architecture for scalability
- Single-command usage similar to Glances
- Configurable refresh rates and metrics collection
- Adaptive UI that works on different terminal sizes

## Quick Start

### One-command usage (like Glances):
```bash
cargo run
```

Or using the provided scripts:
```bash
# PowerShell
.\scripts\check.ps1

# Windows Command Prompt
.\scripts\check.bat

# Linux/macOS Bash
./scripts/check.sh
```

### Individual component usage:
```bash
# Terminal UI only
cargo run -p zek-tui

# Agent only (data collection)
cargo run -p zek-agent

# Gateway only (fleet management)
cargo run -p zek-gateway
```

## Cross-Platform Building

Zek can be built for Linux, Windows, and macOS using Rust's cross-compilation capabilities.

### Prerequisites

- Rust 1.81+
- Cargo package manager
- For cross-compilation: Appropriate target toolchains

### Building for Your Current Platform

To build for your current platform in development mode:
```bash
cargo build
```

To build for your current platform in release mode (optimized):
```bash
cargo build --release
```

### Building for Linux

#### From Linux:
```bash
# Development build
cargo build --target x86_64-unknown-linux-gnu

# Release build
cargo build --release --target x86_64-unknown-linux-gnu
```

#### From Windows/macOS (Cross-compilation):
1. Install the Linux target:
   ```bash
   rustup target add x86_64-unknown-linux-gnu
   ```

2. Install required dependencies (Ubuntu/Debian example):
   ```bash
   sudo apt-get update
   sudo apt-get install gcc libc6-dev
   ```

3. Build:
   ```bash
   # Development build
   cargo build --target x86_64-unknown-linux-gnu
   
   # Release build
   cargo build --release --target x86_64-unknown-linux-gnu
   ```

### Building for Windows

#### From Windows:
```bash
# Development build
cargo build --target x86_64-pc-windows-msvc

# Release build
cargo build --release --target x86_64-pc-windows-msvc
```

#### From Linux/macOS (Cross-compilation):
1. Install the Windows target:
   ```bash
   rustup target add x86_64-pc-windows-msvc
   ```

2. Install required dependencies:
   - On Ubuntu/Debian:
     ```bash
     sudo apt-get install gcc-mingw-w64
     ```

3. Build:
   ```bash
   # Development build
   cargo build --target x86_64-pc-windows-msvc
   
   # Release build
   cargo build --release --target x86_64-pc-windows-msvc
   ```

### Building for macOS

#### From macOS:
```bash
# Development build
cargo build --target x86_64-apple-darwin

# Release build
cargo build --release --target x86_64-apple-darwin
```

#### From Linux/Windows (Cross-compilation):
1. Install the macOS target:
   ```bash
   rustup target add x86_64-apple-darwin
   ```

2. Note: Cross-compilation to macOS is complex and requires specific SDKs. It's recommended to build on macOS directly.

### Creating Standalone Executables

To create standalone executables that can be distributed without requiring Rust to be installed:

```bash
# Build in release mode for standalone distribution
cargo build --release

# The executable will be located at:
# Windows: target/release/zek-cli.exe
# Linux/macOS: target/release/zek-cli
```

### Platform-Specific Considerations

- **Linux**: Best performance and full feature support
- **Windows**: Full feature support with Windows-specific optimizations
- **macOS**: Full feature support (when built on macOS)

## Components

- **zek-tui**: Terminal user interface with real-time metrics visualization
- **zek-agent**: Headless metrics collection agent
- **zek-gateway**: Fleet management gateway for multi-host monitoring
- **zek-cli**: All-in-one command that combines all components

## Configuration

The tool is configured via `configs/zek.toml`:
```toml
[refresh]
interval_ms = 500  # Refresh interval in milliseconds

[collectors]
cpu = true
mem = true
load = true

[exporters]
prometheus = { bind = "0.0.0.0:9100" }

[web]
bind = "0.0.0.0:61208"
```

## Terminal Requirements

For optimal viewing experience:
- **Recommended**: 120 columns × 40 rows or larger for full feature display
- **Minimum**: 80 columns × 20 rows for basic functionality

See [TERMINAL_SETUP.md](TERMINAL_SETUP.md) for detailed instructions on resizing your terminal for optimal viewing.

## Development Scripts

- `.\scripts\dev_run_tui.sh` - Run terminal UI
- `.\scripts\dev_run_agent.sh` - Run agent
- `.\scripts\dev_run_gateway.sh` - Run gateway

## Requirements

- Rust 1.81+
- Cargo package manager

## Usage

1. Clone the repository
2. Navigate to the zek directory
3. Run `cargo run` for the full experience
4. Press `q` or `ESC` to exit

The interface will show:
- CPU usage with visual gauge
- Memory usage with visual gauge (including swap)
- System load averages (1, 5, and 15 minutes)
- Network interface statistics with throughput
- Per-core CPU usage
- Disk usage information
- Sensor temperature data
- GPU utilization (when available)
- Container monitoring (when available)
- Top processes by CPU/Memory usage

## Key Controls

- **Quit**: Press `q` or `ESC`
- **Navigation**: Use arrow keys to navigate tables (in supported versions)

## Comparison with Glances

For a detailed comparison between Zek and Glances, see:
- [ZEK_VS_GLANCES_COMPARISON.md](ZEK_VS_GLANCES_COMPARISON.md) - Comprehensive feature comparison
- [ZEK_ARCHITECTURE.mmd](ZEK_ARCHITECTURE.mmd) - Zek architecture diagram
- [GLANCES_ARCHITECTURE.mmd](GLANCES_ARCHITECTURE.mmd) - Glances architecture diagram
- [ZEK_VS_GLANCES.mmd](ZEK_VS_GLANCES.mmd) - Direct comparison diagram

These diagrams can be viewed using any Mermaid-compatible viewer or editor.

## Roadmap

For future features and enhancements, see [FEATURE_ROADMAP.md](FEATURE_ROADMAP.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.