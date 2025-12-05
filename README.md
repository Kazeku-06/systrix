# Systrix - System Monitor Terminal App

[![CI](https://github.com/yourusername/systrix/workflows/CI/badge.svg)](https://github.com/yourusername/systrix/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Systrix** is a cross-platform system monitoring tool with both CLI and TUI interfaces, built in Rust.

## Features

- 📊 **Real-time monitoring**: CPU, Memory, Disk, Network, Processes
- 🖥️ **Interactive TUI**: Full-screen terminal UI with charts and tables
- ⚡ **CLI commands**: Quick system snapshots and process management
- 🎨 **Multiple themes**: Dark, Light, Dracula
- 🔌 **Plugin system**: Extensible architecture
- 🌐 **Remote monitoring**: Optional remote agent (feature flag)
- 🎯 **Low overhead**: ~3-5% CPU usage at idle
- 🦀 **Cross-platform**: Linux, macOS, Windows

## Installation

### From source

```bash
# Clone repository
git clone https://github.com/yourusername/systrix.git
cd systrix

# Build release binary
cargo build --release

# Install to system
cargo install --path .
```

### Requirements

- Rust 1.70 or later
- For GPU monitoring: NVIDIA drivers with NVML support (optional)

## Usage

### CLI Mode

```bash
# System information summary
systrix info

# List processes (sorted by CPU usage)
systrix ps --sort cpu --limit 20

# Filter processes by name
systrix ps --filter chrome

# Kill a process
systrix kill 1234

# Kill with specific signal
systrix kill 1234 --signal SIGKILL

# Network interfaces snapshot
systrix net

# Disk partitions and usage
systrix disk

# Export system report to JSON
systrix report --output report.json

# Show version
systrix version
```

### TUI Mode

```bash
# Launch interactive TUI
systrix tui

# Custom refresh interval (milliseconds)
systrix tui --refresh-interval 1000
```

#### TUI Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit |
| `1-7` | Switch panels (Overview, Processes, Network, Disk, GPU, Tools, Settings) |
| `Tab` | Next panel |
| `↑↓` | Navigate list |
| `Enter` | Show details |
| `/` | Search processes |
| `k` | Kill selected process |
| `s` | Suspend process |
| `r` | Resume process |
| `+/-` | Renice process |
| `p` | Pause/resume refresh |
| `t` | Toggle theme |

### TUI Screenshots (ASCII)

```
┌──────────────────────── SYSTRIX ────────────────────────┐
│ CPU: ▇▇▇▇▇ 34%  RAM: ▇▇▇ 42%  DISK: ▇▇ 61%  NET: ↑1.2MB/s│
├────────────────────────────────────────────────────────┤
│ [CPU per-core chart]   [Memory chart]   [Disk I/O chart]│
│ [Network chart]        [GPU summary]                   │
└────────────────────────────────────────────────────────┘

Processes (selectable)
PID   USER   NAME            CPU%   MEM%   IO_R   IO_W   THREADS
1234  user   chrome          34.2   5.1    2.1MB  0.3MB     42
2345  root   python3         12.0   1.2    0.0    0.0       8
...
[↑↓ select]  [k Kill] [s Suspend] [r Resume] [Enter details]
```

## Configuration

Configuration file: `config/default.toml`

```toml
[general]
refresh_interval_ms = 500
theme = "dark"

[tui]
show_graphs = true
process_limit = 100

[remote]
enabled = false
port = 8080
token = "your-secret-token"
```

## Building

### Standard build

```bash
cargo build --release
```

### With all features

```bash
cargo build --release --all-features
```

### Cross-compilation

Using [cross](https://github.com/cross-rs/cross):

```bash
# Install cross
cargo install cross

# Build for Linux x86_64
cross build --release --target x86_64-unknown-linux-gnu

# Build for Windows x86_64
cross build --release --target x86_64-pc-windows-gnu

# Build for macOS (requires macOS host)
cargo build --release --target x86_64-apple-darwin
```

## Testing

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run specific test
cargo test monitor_tests
```

## Platform Support & Limitations

### Linux
- ✅ Full support for all features
- ✅ SMART data via `smartctl` (if installed)
- ✅ GPU monitoring (NVIDIA with NVML)

### macOS
- ✅ Core monitoring features
- ⚠️ Limited disk I/O metrics
- ⚠️ GPU monitoring not available

### Windows
- ✅ Core monitoring features
- ⚠️ Some process metrics limited
- ⚠️ SMART data not available
- ⚠️ GPU monitoring limited

## Architecture

```
src/
├── main.rs           # Entry point
├── cli.rs            # CLI argument parsing
├── app.rs            # TUI application state
├── tui/              # TUI components
│   ├── ui.rs         # Layout and rendering
│   ├── event.rs      # Event handling
│   └── panels/       # Individual panels
├── monitor/          # System monitoring backends
│   ├── cpu.rs
│   ├── memory.rs
│   ├── disk.rs
│   ├── network.rs
│   └── process.rs
├── plugins.rs        # Plugin system
├── remote_agent.rs   # Remote monitoring server
└── utils.rs          # Utilities
```

## Performance

- **Idle CPU usage**: ~3-5% on modern hardware
- **Memory footprint**: ~10-20 MB
- **Refresh rate**: Configurable (default 500ms, minimum 100ms)
- **Caching**: Expensive operations (GPU, SMART) cached with 5s TTL

## Security Considerations

- Process management requires appropriate permissions
- Remote agent uses token-based authentication
- Config files should have restricted permissions (0600)
- Cannot kill critical system processes (PID 1, etc.)

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- System info via [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- CLI parsing with [clap](https://github.com/clap-rs/clap)
