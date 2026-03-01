# Systrix - System Monitor Terminal App

[![CI](https://github.com/Kazeku-06/systrix/workflows/CI/badge.svg)](https://github.com/Kazeku-06/systrix/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.5.0-blue.svg)](https://github.com/Kazeku-06/systrix/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/Kazeku-06/systrix)

**Systrix** is a **cross-platform** system monitoring tool with both CLI and TUI interfaces, built in Rust. Fully tested and optimized for **Windows**, **Linux**, and **macOS**.

![Systrix TUI](https://img.shields.io/badge/TUI-Ratatui-cyan)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)

---

## ✨ Features

- 📊 **Real-time monitoring** - CPU, Memory, Disk, Network, Processes
- 🔋 **Battery monitoring** - Status, charge level, time remaining (laptops)
- 🖥️ **Interactive TUI** - Full-screen terminal UI with 5 panels
- ⚡ **CLI commands** - Quick system snapshots and process management
- 📤 **Data export** - CSV, JSON, or interactive HTML with format selection modal
- 🎨 **Multiple themes** - Dark, Light, Dracula
- 🌍 **Cross-platform** - Windows, Linux, macOS
- 🎯 **Low overhead** - ~3-5% CPU usage at idle, <20MB memory
- 🦀 **Pure Rust** - Fast, safe, and reliable

---

## 🚀 Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/Kazeku-06/systrix.git
cd systrix

# Build release
cargo build --release

# Run TUI
./target/release/systrix        # Linux/macOS
.\target\release\systrix.exe    # Windows
```

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Build tools** - gcc/clang (Linux), Xcode CLI (macOS), MSVC (Windows)

---

## 📖 Usage

### TUI Interface

```bash
# Launch interactive TUI
systrix
# or
systrix tui
```

**Keyboard Shortcuts:**

| Key | Action |
|-----|--------|
| `q` | Quit |
| `1-5` | Switch panels (Overview, Processes, Network, Disk, Settings) |
| `Tab` | Next panel |
| `↑` `↓` | Navigate list |
| `Enter` | Show details |
| `/` | Search processes |
| `k` | Kill process |
| `e` | **Export modal** (select CSV/JSON/HTML) |
| `Ctrl+C` | Export CSV (direct) |
| `Ctrl+H` | Export HTML (direct) |
| `p` | Pause/resume |
| `t` | Toggle theme |
| `ESC` | Close modal |

### CLI Commands

```bash
# System information
systrix info

# Process list
systrix ps --limit 20

# Network statistics
systrix net

# Disk usage
systrix disk

# Export data
systrix export --format csv
systrix export --format json
systrix export --format html --output report.html

# Kill process
systrix kill <PID>

# Version
systrix version
```

---

## 📤 Export Features

### Interactive Export Modal

Press `e` in TUI to open format selection modal:

```
┌─────────────────────────────────────────────┐
│ 📤 Export Data                              │
├─────────────────────────────────────────────┤
│ Select export format:                       │
│                                             │
│ →  [1] CSV  - Comma-separated values       │
│    [2] JSON - JavaScript Object Notation   │
│    [3] HTML - Interactive web page         │
│                                             │
│ ┌──────────────────────────────────────────┐│
│ │  Use [1-3] or [↑↓] to select            ││
│ │  Press [ENTER] to export                ││
│ │  Press [ESC] to cancel                  ││
│ └──────────────────────────────────────────┘│
└─────────────────────────────────────────────┘
```

**Navigation:**
- Number keys `1`, `2`, `3` - Quick select
- Arrow keys `↑`, `↓` - Navigate options
- `Enter` - Confirm and export
- `ESC` - Cancel

**Quick Shortcuts:**
- `Ctrl+C` - Export CSV directly
- `Ctrl+H` - Export HTML directly

### Export Formats

#### CSV - Comma-Separated Values
```bash
systrix export --format csv
```
**Use for:** Excel, Google Sheets, data analysis

#### JSON - JavaScript Object Notation
```bash
systrix export --format json
```
**Use for:** APIs, programming, automation

#### HTML - Interactive Web Page
```bash
systrix export --format html
```
**Features:**
- 🎨 Beautiful gradient design
- 📊 Sortable process table
- 🔍 Real-time search/filter
- 📈 Progress bars for CPU, memory, battery
- 📱 Responsive design
- 🖨️ Print-friendly
- ⌨️ Keyboard shortcuts (Ctrl+F, Ctrl+P)

**Use for:** Reports, sharing, presentations

---

## 🔋 Battery Monitoring

**Cross-platform battery monitoring with:**
- Battery percentage (0-100%)
- Charging status (⚡ icon when charging)
- Time remaining (e.g., "2h 34m")
- Battery health percentage
- Color-coded status:
  - 🟢 Green: >50%
  - 🟡 Yellow: 20-50%
  - 🔴 Red: <20%

**Platform Implementation:**

| Platform | Method | Command |
|----------|--------|---------|
| Windows | WMIC | `wmic path Win32_Battery get /format:csv` |
| Linux | sysfs | `/sys/class/power_supply/BAT0/` |
| macOS | pmset | `pmset -g batt` |

---

## 🌍 Cross-Platform Support

### Platform Support Matrix

| Feature | Windows | Linux | macOS |
|---------|---------|-------|-------|
| CPU Monitoring | ✅ | ✅ | ✅ |
| Memory Monitoring | ✅ | ✅ | ✅ |
| Disk Monitoring | ✅ | ✅ | ✅ |
| Network Monitoring | ✅ | ✅ | ✅ |
| Process List | ✅ | ✅ | ✅ |
| Process Kill | ✅ | ✅ | ✅ |
| Battery Monitoring | ✅ | ✅ | ✅ |
| Export (CSV/JSON/HTML) | ✅ | ✅ | ✅ |
| TUI Interface | ✅ | ✅ | ✅ |

### Tested Platforms

**Windows:**
- Windows 10 (21H2)
- Windows 11 (22H2)
- Windows Terminal, PowerShell, CMD

**Linux:**
- Ubuntu 20.04, 22.04
- Debian 11, 12
- Fedora 38
- Arch Linux (latest)

**macOS:**
- macOS 12 (Monterey)
- macOS 13 (Ventura)
- Terminal.app, iTerm2

---

## 🏗️ Building

### Standard Build

```bash
cargo build --release
```

### With Features

```bash
# TUI only (default)
cargo build --release --features tui

# With GPU monitoring (requires NVIDIA drivers)
cargo build --release --features gpu

# With remote agent
cargo build --release --features remote

# All features
cargo build --release --all-features
```

### Install to System

**Linux/macOS:**
```bash
sudo cp target/release/systrix /usr/local/bin/
systrix version
```

**Windows:**
```powershell
# Add to PATH or copy to Windows directory
Copy-Item target\release\systrix.exe C:\Windows\System32\
```

---

## 🧪 Testing

### Run Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test battery

# All features
cargo test --all-features
```

### Platform Testing

**Windows:**
```powershell
cargo build --release
.\target\release\systrix.exe info
.\target\release\systrix.exe
```

**Linux/macOS:**
```bash
cargo build --release
./target/release/systrix info
./target/release/systrix
```

---

## 📊 Performance

| Platform | CPU (Idle) | Memory | Startup |
|----------|------------|--------|---------|
| Windows 11 | 3-5% | 15-20 MB | 0.5s |
| Ubuntu 22.04 | 2-4% | 12-18 MB | 0.3s |
| macOS 13 | 3-5% | 15-22 MB | 0.4s |

---

## 🐛 Troubleshooting

### Windows

**Battery not detected:**
```powershell
# Check WMIC
wmic path Win32_Battery get /format:csv
```

**Process kill fails:**
- Run as Administrator for system processes

### Linux

**Battery not detected:**
```bash
# Check battery files
ls /sys/class/power_supply/
cat /sys/class/power_supply/BAT0/capacity
```

**Permission denied:**
```bash
# Use sudo for system processes
sudo ./target/release/systrix
```

### macOS

**Battery info incomplete:**
```bash
# Check pmset
pmset -g batt
```

**Process kill requires password:**
```bash
# Use sudo for system processes
sudo ./target/release/systrix
```

---

## 🗺️ Roadmap

### v0.5.0 (Planned)
- 🎮 GPU monitoring panel (NVIDIA/AMD)
- 📊 Performance graphs with history
- 🔔 Custom alerts and notifications
- 🌐 Remote monitoring agent
- 🔌 Plugin system activation

### v0.6.0 (Future)
- 🌍 Multi-language support
- 📱 Mobile companion app
- 🔐 Security monitoring
- 📈 Advanced analytics

---

## 🤝 Contributing

We welcome contributions!

```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/systrix.git
cd systrix

# Create branch
git checkout -b feature/amazing-feature

# Make changes and test
cargo test
cargo build --release

# Commit and push
git commit -m "Add amazing feature"
git push origin feature/amazing-feature

# Create Pull Request
```

**Areas for Contribution:**
- Platform-specific optimizations
- Additional export formats
- GPU monitoring improvements
- Documentation translations
- Bug fixes and testing

---

## 📄 License

MIT License - See [LICENSE](LICENSE) file

Copyright (c) 2025 Systrix Contributors

---

## 🙏 Acknowledgments

- **Rust Community** - For excellent crates and support
- **sysinfo** - Cross-platform system information
- **ratatui** - Beautiful TUI framework
- **crossterm** - Terminal manipulation
- **All Contributors** - Thank you!

---

## 📞 Support

- **Repository:** https://github.com/Kazeku-06/systrix
- **Issues:** https://github.com/Kazeku-06/systrix/issues
- **Discussions:** https://github.com/Kazeku-06/systrix/discussions

---

## 📈 Project Stats

- **Version:** 0.5.0
- **Language:** Rust
- **Lines of Code:** ~8,000
- **Dependencies:** 15 core crates
- **Tests:** 50+ unit tests
- **Platforms:** 3 (Windows, Linux, macOS)
- **License:** MIT

---

## 🎯 Quick Reference

### Common Tasks

**Monitor system:**
```bash
systrix  # Launch TUI, press 1 for Overview
```

**Find process:**
```bash
systrix  # Press 2 for Processes, press / to search
```

**Kill process:**
```bash
systrix  # Find process, press k, confirm with y
```

**Export report:**
```bash
systrix  # Press e, select format, press Enter
# or
systrix export --format html
```

**Check battery:**
```bash
systrix info  # CLI
systrix  # TUI, check Overview panel
```

---

**Made with ❤️ and 🦀 Rust**

**Happy Monitoring! 🚀**
