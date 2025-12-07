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

### Basic C