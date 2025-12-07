# Systrix v0.2.0 Release Notes

## 🎉 What's New in v0.2.0

Version 0.2.0 brings major improvements to the TUI experience with complete Network and Disk panels, process search functionality, and detailed process information.

### ✨ New Features

#### 1. Complete Network Panel
- 📊 Full network interface statistics
- 📈 Real-time RX/TX rates
- 📦 Packet counters
- 🔍 Per-interface details

**How to use**: Press `3` in TUI to access Network panel

#### 2. Complete Disk Panel
- 💾 Disk partition table
- 📊 Usage bars with color coding
  - 🔴 Red: >90% used
  - 🟡 Yellow: >75% used
  - ⚪ White: <75% used
- 📁 Filesystem types
- 📍 Mount points

**How to use**: Press `4` in TUI to access Disk panel

#### 3. Process Search
- 🔍 Search processes by name or user
- ⚡ Real-time filtering as you type
- 🎯 Instant results

**How to use**: 
- Press `/` in Processes panel
- Type to search
- Press `ESC` to clear search

#### 4. Process Detail Modal
- 📝 Detailed process information
- 🔢 Full executable path
- 📊 Disk I/O statistics
- 🧵 Thread count
- 👤 User information

**How to use**: 
- Select a process with arrow keys
- Press `Enter` to view details
- Press `ESC` to close

#### 5. Configuration File Support
- ⚙️ Read settings from `config/default.toml`
- 🎨 Customize theme, refresh interval, etc.
- 💾 Persistent preferences

### 🎨 Improvements

- Better modal sizing for different content types
- Improved process panel with search indicator
- Enhanced keyboard navigation
- Better error handling

### 🐛 Bug Fixes

- Fixed process selection with filtered results
- Fixed scroll behavior in process list
- Fixed terminal restoration on exit
- Fixed memory leaks in data collection

---

## 📊 Comparison: v0.1.0 vs v0.2.0

| Feature | v0.1.0 | v0.2.0 |
|---------|--------|--------|
| Network Panel | Stub | ✅ Complete |
| Disk Panel | Stub | ✅ Complete |
| Process Search | ❌ | ✅ Yes |
| Process Details | ❌ | ✅ Yes |
| Config File | ❌ | ✅ Yes |
| Color Coding | Basic | ✅ Enhanced |

---

## 🚀 Upgrade Guide

### From v0.1.0 to v0.2.0

1. **Pull latest changes**:
   ```bash
   git pull origin main
   ```

2. **Rebuild**:
   ```bash
   cargo build --release
   ```

3. **Reinstall** (if installed to system):
   ```bash
   # Linux/macOS
   sudo cp target/release/systrix /usr/local/bin/
   
   # Windows
   # Copy to PATH location
   ```

4. **Verify version**:
   ```bash
   systrix version
   # Should show: systrix v0.2.0
   ```

### Configuration

Create or update `config/default.toml`:

```toml
[general]
refresh_interval_ms = 500
theme = "dark"

[tui]
show_graphs = true
process_limit = 100
```

---

## 📖 New Keyboard Shortcuts

| Key | Action | Panel |
|-----|--------|-------|
| `/` | Start search | Processes |
| `Enter` | Show details | Processes |
| `ESC` | Clear search / Close modal | Any |
| `3` | Network panel | Any |
| `4` | Disk panel | Any |

---

## 🎯 Usage Examples

### Search for Chrome Processes

1. Launch TUI: `systrix tui`
2. Press `2` to go to Processes panel
3. Press `/` to start search
4. Type `chrome`
5. See filtered results instantly

### View Process Details

1. In Processes panel, use `↑↓` to select a process
2. Press `Enter` to view details
3. See full information including:
   - PID, Name, User
   - CPU and Memory usage
   - Disk I/O statistics
   - Thread count
   - Executable path

### Monitor Network Traffic

1. Press `3` to go to Network panel
2. View real-time RX/TX rates
3. See per-interface statistics
4. Monitor packet counts

### Check Disk Usage

1. Press `4` to go to Disk panel
2. See all partitions with usage bars
3. Color-coded warnings for high usage
4. View filesystem types and mount points

---

## 🔧 Technical Changes

### Architecture
- Added `ModalType` enum for different modal types
- Implemented process filtering system
- Enhanced data collection with disk list
- Improved state management

### Performance
- Optimized process filtering
- Reduced memory allocations
- Better caching strategy

### Code Quality
- Added more comprehensive error handling
- Improved code documentation
- Better separation of concerns

---

## 🐛 Known Issues

None reported yet! Please report issues at:
https://github.com/Kazeku-06/systrix/issues

---

## 🙏 Acknowledgments

Thanks to all contributors and users who provided feedback!

---

## 📞 Support

- **Repository**: https://github.com/Kazeku-06/systrix
- **Issues**: https://github.com/Kazeku-06/systrix/issues
- **Documentation**: See README.md

---

**Enjoy Systrix v0.2.0! 🚀**
