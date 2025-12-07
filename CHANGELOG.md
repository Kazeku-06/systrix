# Changelog

All notable changes to Systrix will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- 🔋 **Battery Monitoring** - Display battery status, percentage, and time remaining (laptops)
  - Battery gauge in Overview panel
  - Color-coded battery status (green/yellow/red)
  - Charging indicator with ⚡ icon
  - Time remaining estimation
  - Battery health percentage
  - Cross-platform support (Windows, Linux, macOS)

## [0.2.0] - 2025-12-06

### Added
- ✨ **Complete Network Panel** - Full network interface details with traffic statistics
- ✨ **Complete Disk Panel** - Disk partition table with usage bars and color coding
- ✨ **Process Search** - Press `/` in Processes panel to search by name or user
- ✨ **Process Detail Modal** - Press `Enter` on a process to see detailed information
- ✨ **Advanced Settings Panel** - Comprehensive settings with 5 categories
  - 🎨 Appearance - Theme settings and color coding info
  - ⚡ Performance - Refresh interval and monitoring status
  - 📊 Display - Process limit and graph options
  - ⌨️ Keyboard - Complete keyboard shortcuts reference
  - ℹ️ About - Version info and technology stack
- ✨ **Configuration File Support** - Read settings from config/default.toml
- 📊 Disk usage color coding (red >90%, yellow >75%)
- 🔍 Real-time process filtering as you type
- 📝 Process detail modal shows full executable path, threads, disk I/O
- 🎯 Settings navigation with arrow keys
- ⌨️ Number keys (1-5) in Settings panel to jump to categories
- ⌨️ PageUp/PageDown, Home/End support in all panels

### Changed
- 🎨 Improved Processes panel with search indicator
- 🎨 Better modal sizing for different content types
- 📊 Network panel now shows per-interface statistics
- 📊 Disk panel shows filesystem types and mount points
- 🎨 Settings panel completely redesigned with categories
- ⚡ Improved scroll behavior to work with filtered process lists

### Fixed
- 🐛 Fixed process selection with filtered results
- 🐛 Fixed scroll behavior in process list (now works with filtered results)
- 🐛 Fixed arrow key navigation in Processes and Settings panels
- 🐛 Fixed terminal restoration on exit
- 🐛 Fixed PageUp/PageDown to respect filtered process list
- 🐛 Fixed number keys (1-5) to always switch panels, not Settings categories

## [0.1.0] - 2025-12-05

### Added
- 🎉 Initial release
- ✅ CLI commands (info, ps, kill, net, disk, report, version)
- ✅ Interactive TUI with Overview and Processes panels
- ✅ Real-time monitoring (CPU, Memory, Disk, Network, Processes)
- ✅ Process management with safety checks
- ✅ Multiple themes (Dark, Light, Dracula)
- ✅ Cross-platform support (Linux, macOS, Windows)
- ✅ Async monitoring with Tokio
- ✅ Comprehensive documentation
- ✅ Unit tests and CI/CD

[0.2.0]: https://github.com/Kazeku-06/systrix/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Kazeku-06/systrix/releases/tag/v0.1.0
