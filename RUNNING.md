# Running Zek

## Quick Start

You can now run Zek with a simple command:

```bash
cargo run
```

This will automatically run the `zek-cli` binary which provides the main system monitoring interface.

## Terminal Size Requirements

Zek automatically adapts to your terminal size, but for the best experience:

- **Recommended**: 120 columns × 40 rows or larger for full feature display
- **Minimum**: 80 columns × 20 rows for basic functionality

See [TERMINAL_SETUP.md](TERMINAL_SETUP.md) for detailed instructions on resizing your terminal for optimal viewing.

## Controlling Debug Output

By default, Zek only shows warnings and errors in the terminal. To see more detailed debug information, you can set the `RUST_LOG` environment variable:

```bash
# Show debug messages
RUST_LOG=debug cargo run

# Show info, warnings, and errors
RUST_LOG=info cargo run

# Show only warnings and errors (default)
RUST_LOG=warn cargo run
```

## Alternative Methods

If you prefer to be explicit about which binary to run:

```bash
cargo run --bin zek-cli
```

## Running in Release Mode

For better performance, you can run in release mode:

```bash
cargo run --release
```

## Command Line Usage

Once running, Zek provides a real-time system monitoring interface with the following controls:

- **Quit**: Press `q` or `ESC`
- **Navigation**: Use arrow keys to navigate tables

## Features Displayed

Zek displays comprehensive system information including:

1. **CPU Usage**: Overall CPU usage percentage and per-core usage
2. **Memory**: RAM and swap usage with detailed statistics
3. **System Load**: Load averages (1, 5, and 15 minutes)
4. **Network**: Interface statistics with real-time throughput
5. **Disk**: Mount point usage statistics
6. **Sensors**: System component temperatures
7. **GPU**: Graphics processing unit statistics (framework ready)
8. **Containers**: Docker/container monitoring (framework ready)
9. **Processes**: Top processes by CPU/Memory usage
10. **Process Tree**: Hierarchical process relationships

Zek provides a Glances-like single-command experience for comprehensive system monitoring.