# What's New in Systrix

## Latest Updates (December 7, 2025)

### 🔋 Battery Monitoring Feature - NEW!

Systrix sekarang mendukung monitoring baterai untuk laptop! Fitur ini menampilkan informasi lengkap tentang status baterai Anda secara real-time.

**Fitur Baterai:**
- ✅ Persentase charge baterai (0-100%)
- ✅ Status charging (Charging, Discharging, Full)
- ✅ Estimasi waktu tersisa
- ✅ Battery health percentage
- ✅ Color-coded visual (hijau/kuning/merah)
- ✅ Icon status (⚡ charging, 🔋 normal, 🪫 low)
- ✅ Cross-platform (Windows, Linux, macOS)

**Cara Menggunakan:**
```bash
# Jalankan TUI
.\target\release\systrix.exe tui

# Tekan 1 untuk Overview panel
# Battery gauge akan muncul otomatis jika terdeteksi
```

**Tampilan:**
```
┌─ Battery - Charging ────────────────────────┐
│ ⚡ 85% (2h 15m)                              │
└─────────────────────────────────────────────┘

System Information:
  Battery Health: 95%
```

📖 **Dokumentasi Lengkap**: Lihat [BATTERY_MONITORING.md](BATTERY_MONITORING.md)

---

### 🐛 Bug Fix: Settings Panel Navigation

**Masalah yang Diperbaiki:**
- Angka 1-5 di Settings panel sekarang berfungsi untuk pindah panel
- Sebelumnya angka 1-5 hanya pindah kategori dalam Settings

**Navigasi Baru:**
- `1-5` → Pindah panel (Overview, Processes, Network, Disk, Settings)
- `↑↓` → Navigasi kategori dalam Settings
- `PageUp/PageDown` → Lompat ke kategori pertama/terakhir
- `Home/End` → Lompat ke kategori pertama/terakhir

---

## Version 0.2.0 Features

### ✨ Complete Network Panel
- Full network interface statistics
- Real-time RX/TX rates
- Packet counters
- Per-interface details

### 💾 Complete Disk Panel
- Disk partition table
- Usage bars with color coding
- Filesystem types
- Mount points

### 🔍 Process Search
- Press `/` to search processes
- Real-time filtering
- Search by name or user

### 📝 Process Details
- Press `Enter` on a process
- View full information
- Disk I/O statistics
- Thread count

### ⚙️ Advanced Settings Panel
5 categories:
1. **Appearance** - Theme settings
2. **Performance** - Refresh interval
3. **Display** - Process limit, graphs
4. **Keyboard** - Complete shortcuts
5. **About** - Version and tech stack

---

## Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/Kazeku-06/systrix.git
cd systrix

# Build
cargo build --release

# Run
.\target\release\systrix.exe tui
```

### Basic Commands

```bash
# System info
.\target\release\systrix.exe info

# Process list
.\target\release\systrix.exe ps

# Network stats
.\target\release\systrix.exe net

# Disk usage
.\target\release\systrix.exe disk

# Interactive TUI
.\target\release\systrix.exe tui
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `1-5` | Switch panels |
| `↑↓` | Navigate |
| `/` | Search processes |
| `Enter` | Show details |
| `k` | Kill process |
| `p` | Pause/Resume |
| `t` | Toggle theme |
| `q` | Quit |

---

## What's Coming Next

### Planned Features (v0.3.0+)

**High Priority:**
- [ ] GPU monitoring (NVIDIA/AMD)
- [ ] System logs viewer
- [ ] Performance graphs with history
- [ ] Export data (CSV/JSON)

**Medium Priority:**
- [ ] Custom alerts and notifications
- [ ] Process suspend/resume
- [ ] Remote monitoring agent
- [ ] Plugin system activation
- [ ] Configuration UI

**Community Requests:**
- [ ] Docker container monitoring
- [ ] Temperature sensors
- [ ] Fan speed monitoring
- [ ] Custom themes editor
- [ ] Multi-language support

---

## Recent Changes

### December 7, 2025
- ✅ Added battery monitoring feature
- ✅ Fixed Settings panel navigation
- ✅ Updated documentation

### December 6, 2025
- ✅ Released v0.2.0
- ✅ Complete Network and Disk panels
- ✅ Process search and details
- ✅ Advanced Settings panel

### December 5, 2025
- ✅ Released v0.1.0
- ✅ Initial CLI and TUI implementation
- ✅ Basic monitoring features

---

## Documentation

### User Guides
- [README.md](README.md) - Main overview
- [QUICKSTART.md](QUICKSTART.md) - Quick start guide
- [INSTALLATION.md](INSTALLATION.md) - Installation instructions
- [RUNNING.md](RUNNING.md) - Running and PATH setup
- [EXAMPLES.md](EXAMPLES.md) - Usage examples

### Feature Guides
- [BATTERY_MONITORING.md](BATTERY_MONITORING.md) - Battery feature guide
- [SETTINGS_PANEL_GUIDE.md](SETTINGS_PANEL_GUIDE.md) - Settings panel guide

### Technical Docs
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical architecture
- [BUILD.md](BUILD.md) - Build instructions
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [CHANGELOG.md](CHANGELOG.md) - Version history

---

## Support

- **Repository**: https://github.com/Kazeku-06/systrix
- **Issues**: https://github.com/Kazeku-06/systrix/issues
- **Clone**: `git clone https://github.com/Kazeku-06/systrix.git`

---

## Stats

- **Version**: 0.2.0 (+ battery feature)
- **Lines of Code**: ~5,000+
- **Files**: 40+
- **Platforms**: Windows, Linux, macOS
- **Language**: Rust 🦀
- **License**: MIT

---

**Terima kasih telah menggunakan Systrix! 🚀**

Jika ada pertanyaan atau saran, silakan buat issue di GitHub repository.
