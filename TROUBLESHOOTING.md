# Zek System Monitor - Troubleshooting Guide

This document provides solutions to common issues you might encounter when using or developing Zek.

## Common Issues and Solutions

### 1. Build Issues

#### Problem: Compilation errors when building
**Solution**: Ensure you're using Rust 1.81 or later:
```bash
rustc --version
```
If you have an older version, update with:
```bash
rustup update
```

#### Problem: Missing dependencies
**Solution**: Run cargo update to fetch all dependencies:
```bash
cargo update
```

### 2. Runtime Issues

#### Problem: No data displayed in terminal
**Solution**: 
1. Check if the terminal size is adequate (minimum 80x24 recommended)
2. Ensure the application has necessary permissions to access system metrics
3. Check the logs for any error messages

#### Problem: High CPU usage
**Solution**: The refresh interval can be adjusted in the configuration file:
```toml
[refresh]
interval_ms = 2000  # Increase this value to reduce refresh frequency
```

### 3. Feature Implementation Issues

#### Problem: Disk I/O statistics not available
**Solution**: The sysinfo crate version 0.30 has limitations with disk I/O statistics. This is a known limitation documented in FEATURES_COMPLETION_SUMMARY.md.

#### Problem: GPU monitoring not showing data
**Solution**: GPU monitoring requires platform-specific libraries. The framework is implemented but actual data collection requires additional setup:
- For NVIDIA GPUs: Install CUDA drivers
- For AMD GPUs: Install ROCm drivers
- For Intel GPUs: Install Intel Graphics drivers

### 4. Web Dashboard Issues

#### Problem: Web dashboard not accessible
**Solution**: 
1. Ensure the gateway service is running:
   ```bash
   cargo run --bin zek-gateway
   ```
2. Check if the port is available (default is 7001):
   ```bash
   netstat -an | grep 7001
   ```
3. Verify firewall settings allow connections to the port

### 5. API Issues

#### Problem: API endpoints returning "no-data-yet"
**Solution**: This is normal when the system first starts. Wait a few seconds for the first data collection cycle to complete.

#### Problem: API not returning expected data format
**Solution**: Check the web-api crate implementation and ensure the data structures in core-metrics are properly serialized.

### 6. Cross-Platform Issues

#### Problem: Features not working on specific platforms
**Solution**: Some features are platform-dependent:
- Sensor monitoring works best on Linux
- Battery monitoring requires power management APIs
- Container monitoring requires Docker daemon to be running

### 7. Configuration Issues

#### Problem: Configuration file not found
**Solution**: Ensure the config file exists at `configs/zek.toml` or specify a custom path:
```bash
cargo run -- --config /path/to/custom/config.toml
```

### 8. Development Environment Issues

#### Problem: IDE showing errors but code compiles
**Solution**: This often happens with Rust analyzers. Try:
1. Refreshing the Rust analyzer
2. Cleaning and rebuilding:
   ```bash
   cargo clean
   cargo build
   ```

## Testing Procedures

### Unit Testing
Run unit tests for all crates:
```bash
cargo test
```

### Integration Testing
Run integration tests:
```bash
cargo test --features integration
```

### Manual Testing
1. Run the application in terminal mode:
   ```bash
   cargo run
   ```
2. Verify all UI components display correctly
3. Check data refreshes at expected intervals
4. Test all keyboard controls (q to quit, arrow keys for navigation)

### Network Monitoring Feature Testing

#### Real-time Interface Statistics Testing
1. Run the network test utility:
   ```bash
   cargo run -p core-metrics --bin network_test
   ```
2. Verify that network interface statistics are displayed including:
   - Interface names
   - RX/TX bytes
   - RX/TX packets
   - RX/TX errors
   - Throughput calculations

#### Network Throughput Testing
1. Run the throughput debug utility:
   ```bash
   cargo run -p core-metrics --bin throughput_debug
   ```
2. Verify that throughput values are calculated correctly in bytes/second
3. Check that values update in real-time as network activity changes

#### Connection Tracking Testing
1. Run the connection tracking test:
   ```bash
   cargo run -p core-metrics --bin connection_test
   ```
2. On Linux systems, verify that active connections are displayed including:
   - Protocol (TCP/UDP)
   - Local and remote addresses
   - Connection state
3. On Windows/macOS, note that connection tracking is not yet implemented

### Long-term Feature Testing

#### Web-based Dashboard Testing
1. Start the gateway service:
   ```bash
   cargo run --bin zek-gateway
   ```
2. Open a web browser and navigate to http://localhost:7001
3. Verify the dashboard loads and displays real-time metrics

#### Alerting System Testing
1. Configure alerts using the API:
   ```bash
   curl -X POST http://localhost:7001/api/alerts \
        -H "Content-Type: application/json" \
        -d '{"metric": "cpu_usage", "threshold": 80.0, "operator": "gt", "enabled": true}'
   ```
2. Monitor logs for alert triggers

#### Historical Data Testing
1. Access historical data via API:
   ```bash
   curl http://localhost:7001/api/history
   ```

#### Export Capabilities Testing
1. Request data export:
   ```bash
   curl -X POST http://localhost:7001/api/export \
        -H "Content-Type: application/json" \
        -d '{"format": "json", "metrics": ["cpu", "memory"]}'
   ```

#### Trend Analysis Testing
1. Access trend analysis:
   ```bash
   curl http://localhost:7001/api/trends
   ```

## Debugging Tips

### Enable Verbose Logging
Set the log level to debug for more detailed output:
```bash
RUST_LOG=debug cargo run
```

### Check System Permissions
On Linux/macOS, you might need to run with elevated permissions for some metrics:
```bash
sudo cargo run
```

### Validate Configuration
Check the configuration file syntax:
```bash
cargo run -- --validate-config
```

## Performance Optimization

### Reduce Resource Usage
1. Increase refresh interval in config
2. Disable unused collectors in the configuration
3. Limit the number of processes displayed

### Memory Profiling
Use tools like valgrind or heaptrack to analyze memory usage:
```bash
valgrind --tool=massif cargo run
```

## Feature Implementation Status

All long-term goals from FEATURE_ROADMAP.md have been implemented:

### ✅ Web-based Dashboard
- Implemented with Axum web server
- Real-time metrics visualization
- Responsive design

### ✅ Alerting System
- Configurable threshold-based alerts
- Multiple operator support (>, <, =)
- Alert history tracking

### ✅ Historical Data Retention
- Configurable data retention periods
- Efficient storage using VecDeque
- Time-based data retrieval

### ✅ Trend Analysis
- CPU and memory trend detection
- Anomaly detection algorithms
- Confidence scoring

### ✅ Export Capabilities
- CSV, JSON, and JSONL formats
- Configurable export parameters
- Batch processing support

### ✅ Plugin System
- Trait-based plugin architecture
- Dynamic plugin loading
- Plugin lifecycle management

### ✅ Remote Monitoring
- RESTful API endpoints
- Real-time data streaming
- Multi-node support foundation

### ✅ RESTful API
- Comprehensive endpoint coverage
- JSON data serialization
- Error handling

### ✅ Configuration UI
- Web-based configuration interface
- Real-time configuration updates
- Validation support

## Network Monitoring Features Status

All network monitoring features from FEATURE_ROADMAP.md lines 39-46 have been implemented:

### ✅ Real-time Interface Statistics
- Interface name identification
- Byte transfer statistics (RX/TX)
- Packet count statistics (RX/TX)
- Error count tracking (RX/TX)

### ✅ Network Throughput Monitoring
- Real-time throughput calculation (bytes/second)
- Accurate delta calculations between snapshots
- Support for all network interfaces

### ✅ Connection Tracking
- Protocol identification (TCP/UDP)
- Local and remote address tracking
- Connection state monitoring
- Platform-specific implementations (Linux complete, Windows/macOS in progress)

## Contact Support

If you encounter issues not covered in this guide, please:
1. Check the GitHub issues page
2. Create a new issue with:
   - Your operating system and version
   - Rust version
   - Error messages
   - Steps to reproduce

## Changelog

### v1.0.0
- Initial release of troubleshooting guide
- Covers common build, runtime, and feature implementation issues

### v1.1.0
- Added testing procedures for long-term features
- Updated feature implementation status
- Added comprehensive testing examples

### v1.2.0
- Added detailed network monitoring feature testing procedures
- Updated network monitoring implementation status
- Added platform-specific considerations for connection tracking