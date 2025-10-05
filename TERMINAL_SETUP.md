# Terminal Setup for Optimal Zek Viewing

Zek System Monitor automatically adjusts its layout based on your terminal size, but for the best experience, we recommend specific terminal dimensions.

## Recommended Terminal Sizes

### For Full Feature Display (Recommended)
- **Minimum Size**: 120 columns × 40 rows
- **Optimal Size**: 150 columns × 50 rows or larger

This size allows Zek to display all system monitoring sections with full detail:
- CPU and Memory usage gauges
- System load averages
- Per-core CPU usage
- Network interface details with throughput
- Disk usage statistics
- Sensor temperature readings
- GPU utilization (when available)
- Container monitoring (when available)
- Top processes with detailed resource usage

### For Medium Screens
- **Minimum Size**: 100 columns × 30 rows

This size uses a compact layout that still shows all features but with reduced row heights:
- Combined sections where appropriate
- Smaller table heights
- Streamlined information display

### For Small Terminals
- **Minimum Size**: 80 columns × 20 rows

This size uses a minimal layout that combines related sections:
- CPU/Memory combined with Load/Network
- Network/Disk data combined
- Sensors/GPU combined
- Containers/Processes combined

## How to Resize Your Terminal

### Windows Terminal/Command Prompt
1. Click and drag the window edges to resize
2. Right-click the title bar → Properties → Layout tab
3. Adjust "Window Size" columns and rows values
4. Click OK to apply

### PowerShell
1. Click and drag the window edges to resize
2. Right-click the title bar → Properties → Layout tab
3. Adjust "Window Size" columns and rows values
4. Click OK to apply

### Linux/MacOS Terminal
1. Click and drag the window edges to resize
2. For precise control, use terminal-specific settings:
   - GNOME Terminal: Edit → Preferences → Profiles → General → Default size
   - iTerm2: Cmd+O → Window tab → Set columns and rows
   - Kitty: Configure `kitty.conf` with `initial_window_width` and `initial_window_height`

## Pro Tips for Better Viewing

1. **Use a modern terminal**: Zek works best with terminals that support full ANSI color and cursor positioning
2. **Font selection**: Use a monospace font like Consolas, Monaco, or Source Code Pro for best alignment
3. **Fullscreen mode**: Press F11 (Windows/Linux) or Cmd+Ctrl+F (Mac) to maximize your terminal
4. **Zoom out**: Use Ctrl+- (or Cmd+- on Mac) to fit more content on screen

## Zek's Adaptive Layout

Zek automatically detects your terminal size and adjusts the layout:
- Large terminals (40+ rows): Full feature display with generous spacing
- Medium terminals (20-39 rows): Compact layout with reduced spacing
- Small terminals (<20 rows): Minimal layout with combined sections

The application will always display all available information, just with different layouts based on available space.