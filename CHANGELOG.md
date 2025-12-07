# Changelog

All notable changes to Systrix will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-06

### Added
- ✨ **Complete Network Panel** - Full network interface details with traffic statistics
- ✨ **Complete Disk Panel** - Disk partition table with usage bars and color coding
- ✨ **Process Search** - Press `/` in Processes panel to search by name or user
- ✨ **Process Detail Modal** - Press `Enter` on a process to see detailed information
- ✨ **Configuration File Support** - Read settings from config/default.toml
- 📊 Disk usage color coding (red >90%, yellow >75%)
- 🔍 Real-time process filtering as you type
- 📝 Process detail modal shows full executable path, threads, disk I/O

### Changed
- 🎨 Improved Processes panel with search indicator
- 🎨 Better modal sizing for different content types
- 📊 Network panel now shows per-interface statistics
- 📊 Disk panel shows filesystem types and mount points

### Fixed
- 🐛 Fixed process selection with filtered results
- 🐛 Fixed scroll behavior in process list
- 🐛 Fixed terminal restoration on exit

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
