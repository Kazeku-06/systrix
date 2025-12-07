# Battery Parsing Fix

## Problem
Battery percentage showing 1% instead of actual 81%

## Root Cause
Wrong CSV parsing in Windows implementation:
- Was reading line index 2 (3rd line) instead of line index 1 (2nd line)
- Was reading wrong column indices for battery data

## WMIC Output Format
```
Node,BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime
NOPALLL,1,80,129
```

**Line 0**: Header (Node,BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime)
**Line 1**: Data (NOPALLL,1,80,129)

**Columns:**
- parts[0] = Node name (NOPALLL)
- parts[1] = BatteryStatus (1 = Discharging)
- parts[2] = EstimatedChargeRemaining (80 = 80%)
- parts[3] = EstimatedRunTime (129 = 129 minutes)

## Fix Applied

### Before (Wrong):
```rust
if let Some(data_line) = lines.get(2) {  // ❌ Line 2 doesn't exist
    let parts: Vec<&str> = data_line.split(',').collect();
    
    if parts.len() >= 3 {
        let battery_status = parts[0].trim().parse::<u32>().unwrap_or(0);  // ❌ Reading Node name
        let percentage = parts[1].trim().parse::<f32>().unwrap_or(0.0);    // ❌ Reading BatteryStatus
        let time_remaining = parts[2].trim().parse::<u64>().ok();          // ❌ Reading Percentage
```

### After (Correct):
```rust
if let Some(data_line) = lines.get(1) {  // ✅ Line 1 (data line)
    let parts: Vec<&str> = data_line.split(',').collect();
    
    if parts.len() >= 4 {
        let battery_status = parts[1].trim().parse::<u32>().unwrap_or(0);  // ✅ BatteryStatus
        let percentage = parts[2].trim().parse::<f32>().unwrap_or(0.0);    // ✅ Percentage
        let time_remaining = parts[3].trim().parse::<u64>().ok();          // ✅ Time
```

## Testing

### Manual Test
```bash
# Check WMIC output
wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime /format:csv

# Expected output:
# Node,BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime
# NOPALLL,1,80,129
```

### After Build
```bash
# Rebuild
cargo build --release

# Run TUI
.\target\release\systrix.exe tui

# Press 1 for Overview
# Battery should now show correct percentage (80% instead of 1%)
```

## Expected Result

**Before Fix:**
```
┌─ Battery - Discharging ─────────────────────┐
│ 🔋 1% (129m)                                 │  ❌ Wrong!
└─────────────────────────────────────────────┘
```

**After Fix:**
```
┌─ Battery - Discharging ─────────────────────┐
│ 🔋 80% (2h 9m)                               │  ✅ Correct!
└─────────────────────────────────────────────┘
```

## Status
- ✅ Bug identified
- ✅ Fix applied
- ⏳ Build in progress
- ⏳ Testing pending

---

**Note**: Setelah build selesai, silakan test untuk memverifikasi bahwa battery percentage sekarang menampilkan nilai yang benar.
