# Settings Panel Guide

## Overview

The Settings panel (press `5` in TUI) provides comprehensive configuration options and information about Systrix.

## Navigation

- **↑↓ Arrow Keys** - Navigate between categories
- **1-5 Number Keys** - Jump to specific category
- **ESC** - Return to previous panel

## Categories

### 1. Appearance 🎨

View and manage theme settings.

**Features:**
- Current theme display
- Available themes list (Dark, Light, Dracula)
- Color coding explanation
- Theme switching instructions

**Quick Actions:**
- Press `t` to cycle through themes

**Color Coding:**
- 🔴 **Red** - Critical (>90% usage)
- 🟡 **Yellow** - Warning (>75% usage)
- ⚪ **White/Green** - Normal (<75% usage)

---

### 2. Performance ⚡

Monitor and optimize performance settings.

**Information Displayed:**
- Current refresh interval (in milliseconds)
- Monitoring status (Running/Paused)
- Performance recommendations
- CPU usage target

**Refresh Interval Guide:**
- **100-250ms** - High frequency (higher CPU usage)
- **500ms** - Default (balanced) ⭐
- **1000-2000ms** - Low frequency (lower CPU usage)

**Quick Actions:**
- Press `p` to pause/resume monitoring

**Performance Tips:**
- Increase refresh interval to reduce CPU usage
- Pause monitoring when not actively viewing
- Target: 3-5% CPU usage at idle

---

### 3. Display 📊

Configure display options and view panel information.

**Settings:**
- Process limit (default: 100)
- Show graphs (enabled/disabled)
- Show per-core CPU (enabled/disabled)

**Panel Overview:**
1. **Overview** - System summary with gauges
2. **Processes** - Process list with search
3. **Network** - Network interface statistics
4. **Disk** - Disk partition information
5. **Settings** - This panel

**Display Features:**
- Real-time updates
- Color-coded warnings
- Sortable columns
- Search and filter

---

### 4. Keyboard ⌨️

Complete keyboard shortcuts reference.

#### Navigation
| Key | Action |
|-----|--------|
| `q` | Quit application |
| `1-5` | Switch to panel |
| `Tab` | Next panel |
| `↑↓` | Navigate list |
| `PageUp/PageDown` | Scroll by page |
| `Home/End` | Jump to top/bottom |

#### Actions
| Key | Action |
|-----|--------|
| `Enter` | Show process details |
| `k` | Kill selected process |
| `/` | Search processes |
| `Esc` | Cancel/Clear search |

#### Settings
| Key | Action |
|-----|--------|
| `p` | Pause/Resume monitoring |
| `t` | Toggle theme |

#### Tips
- Use search (`/`) to quickly find processes
- Pause (`p`) to examine data without updates
- Press `Enter` for detailed process info

---

### 5. About ℹ️

Information about Systrix.

**Displays:**
- Application name and tagline
- Current version
- Technology stack
- Features list
- Repository link
- License information

**Technology Stack:**
- 🦀 **Rust** - Programming language
- **ratatui** - Terminal UI framework
- **tokio** - Async runtime
- **sysinfo** - System information
- **clap** - CLI argument parsing

**Features:**
- ✓ Real-time system monitoring
- ✓ Interactive TUI with multiple panels
- ✓ Process management
- ✓ Network and disk monitoring
- ✓ Multiple themes
- ✓ Cross-platform (Linux, macOS, Windows)

---

## Customization

### Config File

Settings can be customized via `config/default.toml`:

```toml
[general]
refresh_interval_ms = 500
theme = "dark"

[tui]
show_graphs = true
process_limit = 100
show_per_core_cpu = true

[monitoring]
cache_ttl_seconds = 5
```

### Theme Customization

Available themes:
1. **Dark** (default) - Cyan accents on dark background
2. **Light** - Blue accents on light background
3. **Dracula** - Magenta accents with Dracula colors

Cycle themes with `t` key.

---

## Tips & Tricks

### Performance Optimization

1. **Reduce CPU Usage:**
   - Increase refresh interval to 1000ms or higher
   - Pause monitoring when not viewing
   - Close unused terminal windows

2. **Improve Responsiveness:**
   - Decrease refresh interval to 250ms
   - Keep process limit reasonable (50-100)

3. **Balance:**
   - Use default 500ms for best balance
   - Enable/disable graphs based on preference

### Workflow Tips

1. **Quick Navigation:**
   - Use number keys (1-5) to jump between panels
   - Use Tab for sequential navigation

2. **Process Management:**
   - Use `/` to search for specific processes
   - Press `Enter` to view detailed information
   - Press `k` to kill (with confirmation)

3. **Monitoring:**
   - Press `p` to pause and examine data
   - Use color coding to identify issues quickly
   - Check Settings > Performance for status

---

## Troubleshooting

### High CPU Usage

**Problem:** Systrix using too much CPU

**Solutions:**
1. Increase refresh interval (Settings > Performance)
2. Pause monitoring when not needed
3. Reduce process limit (Settings > Display)

### Display Issues

**Problem:** UI not rendering correctly

**Solutions:**
1. Ensure terminal size is at least 80x24
2. Try different theme (press `t`)
3. Check terminal supports ANSI colors

### Keyboard Not Working

**Problem:** Keyboard shortcuts not responding

**Solutions:**
1. Ensure not in search mode (press ESC)
2. Check if modal is open (press ESC to close)
3. Verify terminal focus

---

## Keyboard Shortcuts Quick Reference

```
Navigation:
  q       - Quit
  1-5     - Switch panel
  Tab     - Next panel
  ↑↓      - Navigate
  PgUp/Dn - Scroll page
  Home/End- Jump top/bottom

Actions:
  Enter   - Details
  k       - Kill process
  /       - Search
  Esc     - Cancel

Settings:
  p       - Pause/Resume
  t       - Toggle theme
```

---

## Support

For more information:
- **Repository**: https://github.com/Kazeku-06/systrix
- **Issues**: https://github.com/Kazeku-06/systrix/issues
- **Documentation**: See README.md

---

**Happy Monitoring! 🚀**
