# Battery Monitoring Feature

## Overview

Systrix now includes comprehensive battery monitoring for laptops and portable devices. The battery information is displayed in the Overview panel with real-time updates.

## Features

### 🔋 Battery Status Display

- **Battery Percentage** - Current charge level (0-100%)
- **Charging Status** - Shows if battery is charging, discharging, or fully charged
- **Time Remaining** - Estimated time until full charge or empty (when available)
- **Battery Health** - Overall battery condition (0-100%)
- **Visual Indicators** - Color-coded gauge and status icons

### 🎨 Visual Indicators

**Status Icons:**
- ⚡ - Charging
- 🔋 - Discharging (normal)
- 🪫 - Low battery

**Color Coding:**
- 🟢 **Green** (75-100%) - Good battery level
- 🟡 **Yellow** (30-74%) - Moderate battery level
- 🟠 **Orange** (15-29%) - Low battery warning
- 🔴 **Red** (<15%) - Critical battery level
- 🟢 **Green** (Charging) - Battery is charging regardless of level

## Platform Support

### ✅ Windows
- Uses WMIC (Windows Management Instrumentation Command-line)
- Provides: percentage, status, time remaining
- Automatically detects battery presence

### ✅ Linux
- Reads from `/sys/class/power_supply/BAT0` or `/BAT1`
- Provides: percentage, status, time remaining, health, vendor
- Supports multiple battery detection

### ✅ macOS
- Uses `pmset` and `system_profiler` commands
- Provides: percentage, status, time remaining, health
- Apple battery information

## Display Location

Battery information appears in the **Overview panel** (press `1` in TUI):

```
┌─ Battery - Charging ────────────────────────┐
│ ⚡ 85% (2h 15m)                              │
└─────────────────────────────────────────────┘

System Information:
  Battery Health: 95%
  Battery Vendor: Apple
```

## Usage

### TUI Mode

1. Launch TUI:
   ```bash
   systrix tui
   ```

2. Navigate to Overview panel (press `1`)

3. Battery gauge appears automatically if battery is detected

4. Information updates every refresh interval (default: 500ms)

### CLI Mode

Battery information is included in the `info` command:

```bash
systrix info
```

Output includes battery status if present:
```
Battery:
  Status: Charging
  Level: 85%
  Time Remaining: 2h 15m
  Health: 95%
```

## Technical Details

### Data Structure

```rust
pub struct BatteryInfo {
    pub is_present: bool,        // Battery detected
    pub percentage: f32,          // Charge level (0-100)
    pub is_charging: bool,        // Currently charging
    pub is_plugged: bool,         // AC power connected
    pub time_remaining: Option<u64>, // Seconds until full/empty
    pub health: f32,              // Battery health (0-100)
    pub status: String,           // Status text
    pub technology: String,       // Battery type (Li-ion, etc)
    pub vendor: String,           // Manufacturer
}
```

### Platform-Specific Implementation

**Windows:**
- Command: `wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime`
- Parses CSV output
- Battery status codes: 1=Discharging, 2=Charging, 3=Full, 4=Low, 5=Critical

**Linux:**
- Reads sysfs files: `/sys/class/power_supply/BAT*/`
- Files: `capacity`, `status`, `energy_now`, `power_now`, `energy_full`, `energy_full_design`
- Calculates time remaining from energy and power values
- Calculates health from full vs design capacity

**macOS:**
- Command: `pmset -g batt` for status and time
- Command: `system_profiler SPPowerDataType` for health
- Parses text output with regex patterns

## Configuration

Battery monitoring is always enabled when a battery is detected. No configuration needed.

To adjust refresh rate (affects all monitoring):

```toml
# config/default.toml
[general]
refresh_interval_ms = 500  # Update every 500ms
```

## Troubleshooting

### Battery Not Detected

**Desktop PC:**
- Normal - desktops don't have batteries
- Battery section won't appear in Overview

**Laptop:**
- Check battery is properly connected
- Verify battery drivers are installed
- Try restarting Systrix

### Incorrect Time Remaining

**"Time remaining not available":**
- Some systems don't provide this data
- May appear when battery is fully charged
- May appear when power consumption is very low

**Inaccurate estimates:**
- Time remaining is an estimate based on current power draw
- Changes with system load
- More accurate after a few minutes of stable usage

### Permission Issues (Linux)

If battery info not showing on Linux:

```bash
# Check if battery files are readable
ls -la /sys/class/power_supply/BAT0/

# Should show readable files
# If not, check permissions
```

### Health Always 100%

**Windows:**
- Battery health not easily available via WMIC
- Shows 100% by default
- Use manufacturer tools for accurate health

**Linux/macOS:**
- Health calculated from design vs current capacity
- Should show accurate values

## Examples

### Fully Charged Laptop
```
┌─ Battery - Fully Charged ──────────────────┐
│ 🔋 100%                                     │
└─────────────────────────────────────────────┘
```

### Charging
```
┌─ Battery - Charging ────────────────────────┐
│ ⚡ 45% (1h 30m)                              │
└─────────────────────────────────────────────┘
```

### Low Battery
```
┌─ Battery - Discharging ─────────────────────┐
│ 🪫 12% (25m)                                 │
└─────────────────────────────────────────────┘
```

### Critical Battery
```
┌─ Battery - Critical ────────────────────────┐
│ 🪫 5% (10m)                                  │
└─────────────────────────────────────────────┘
```

## Future Enhancements

Potential improvements for future versions:

- [ ] Battery history graph
- [ ] Power consumption trends
- [ ] Battery wear level tracking
- [ ] Multiple battery support
- [ ] Battery alerts/notifications
- [ ] Power profile recommendations
- [ ] Detailed battery statistics panel

## API Reference

### Functions

```rust
// Get current battery information
pub async fn get_battery_info() -> Result<BatteryInfo>

// Format time in human-readable format
pub fn format_time_remaining(seconds: u64) -> String

// Get appropriate color for battery level
pub fn get_battery_color(percentage: f32, is_charging: bool) -> Color

// Get battery icon (unused in current version)
pub fn get_battery_icon(percentage: f32, is_charging: bool) -> &'static str
```

## Related Files

- `src/monitor/battery.rs` - Battery monitoring implementation
- `src/tui/panels/overview.rs` - Battery display in UI
- `src/tui/ui.rs` - Battery data management

## Support

For issues or questions about battery monitoring:
- **Repository**: https://github.com/Kazeku-06/systrix
- **Issues**: https://github.com/Kazeku-06/systrix/issues

---

**Battery monitoring makes Systrix perfect for laptop users! 🔋⚡**
