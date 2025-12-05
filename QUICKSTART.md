# Systrix Quick Start Guide

## Installation

### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/systrix.git
cd systrix

# Build release binary
cargo build --release

# The binary will be at: target/release/systrix
```

### Option 2: Install with Cargo

```bash
cargo install --path .
```

## First Run

### Try the CLI

```bash
# System information
./target/release/systrix info

# List top 10 processes by CPU
./target/release/systrix ps --limit 10

# Network interfaces
./target/release/systrix net

# Disk usage
./target/release/systrix disk
```

### Launch the TUI

```bash
# Start interactive TUI
./target/release/systrix tui

# With custom refresh rate (1 second)
./target/release/systrix tui --refresh-interval 1000
```

## TUI Quick Reference

### Navigation
- `q` - Quit
- `1` - Overview panel
- `2` - Processes panel
- `3` - Network panel
- `4` - Disk panel
- `5` - Settings panel
- `Tab` - Next panel
- `↑↓` - Navigate list
- `PageUp/PageDown` - Scroll by page
- `Home/End` - Jump to top/bottom

### Actions
- `k` - Kill selected process (with confirmation)
- `p` - Pause/resume refresh
- `t` - Toggle theme (Dark → Light → Dracula)
- `Esc` - Cancel action/close modal

## Common Tasks

### Monitor Specific Process

```bash
# Watch a specific process
systrix ps --filter chrome --limit 5

# Continuous monitoring (Linux/macOS)
watch -n 1 'systrix ps --filter myapp --limit 1'
```

### Kill a Process

```bash
# Interactive (asks for confirmation)
systrix kill 1234

# Force kill without confirmation
systrix kill 1234 --signal SIGKILL --force
```

### Export System Report

```bash
# Export to JSON
systrix report --output system-report.json

# View the report
cat system-report.json | jq .
```

### Run with Elevated Privileges

```bash
# Linux/macOS
sudo systrix tui

# Windows (PowerShell as Administrator)
.\target\release\systrix.exe tui
```

## Troubleshooting

### Build Errors

**Error: `link.exe` not found (Windows)**
```
Install Visual Studio Build Tools:
https://visualstudio.microsoft.com/downloads/
Select "Desktop development with C++"
```

**Error: `ratatui` not found**
```bash
# Enable TUI feature (it's default, but just in case)
cargo build --release --features tui
```

### Runtime Issues

**Permission denied when killing process**
```bash
# Run with elevated privileges
sudo systrix kill 1234
```

**TUI not displaying correctly**
```
- Ensure terminal supports ANSI colors
- Try a different terminal (iTerm2, Windows Terminal, etc.)
- Minimum terminal size: 80x24
```

**GPU not detected**
```
- Install NVIDIA drivers with NVML support
- Build with GPU feature: cargo build --release --features gpu
- GPU monitoring only works on Linux with NVIDIA GPUs
```

## Configuration

Edit `config/default.toml`:

```toml
[general]
refresh_interval_ms = 500
theme = "dark"

[tui]
show_graphs = true
process_limit = 100

[monitoring]
cache_ttl_seconds = 5
enable_gpu = true
```

## Examples

### Monitor System During Load Test

```bash
# Terminal 1: Run your application
./my-app

# Terminal 2: Monitor in real-time
systrix tui
```

### Automated Reporting

```bash
# Add to crontab for hourly reports
0 * * * * /usr/local/bin/systrix report --output /var/log/systrix/report-$(date +\%Y\%m\%d-\%H).json
```

### Find Memory-Hungry Processes

```bash
# Sort by memory usage
systrix ps --sort mem --limit 10
```

### Monitor Network Activity

```bash
# Watch network interfaces
watch -n 1 'systrix net'
```

## Getting Help

```bash
# General help
systrix --help

# Command-specific help
systrix ps --help
systrix kill --help
systrix tui --help
```

## Next Steps

1. Read [README.md](README.md) for detailed documentation
2. Check [EXAMPLES.md](EXAMPLES.md) for more usage examples
3. See [PLATFORM_NOTES.md](PLATFORM_NOTES.md) for platform-specific info
4. Review [BUILD.md](BUILD.md) for advanced build options

## Tips & Tricks

### Performance
- Increase refresh interval to reduce CPU usage: `--refresh-interval 1000`
- Use CLI commands for scripting instead of parsing TUI output
- Export reports for historical analysis

### Workflow
- Use `systrix ps --filter` to quickly find processes
- Press `p` in TUI to pause and examine data
- Use themes to reduce eye strain (press `t` to cycle)

### Scripting
```bash
# Get current CPU usage as JSON
systrix report --output /tmp/report.json
cat /tmp/report.json | jq '.cpu.global_usage'

# Kill all processes matching pattern
systrix ps --filter myapp | tail -n +2 | awk '{print $1}' | \
  xargs -I {} systrix kill {} --force
```

## Support

- **Issues**: https://github.com/yourusername/systrix/issues
- **Discussions**: https://github.com/yourusername/systrix/discussions
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Happy Monitoring! 🚀**
