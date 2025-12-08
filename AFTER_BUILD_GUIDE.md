# After Build - Quick Start Guide

## ✅ Build Complete!

Binary sudah siap di: `.\target\release\systrix.exe`

## 🚀 What to Do Next

### 1. Test Basic Functionality

**Run TUI:**
```bash
.\target\release\systrix.exe tui
```

**Run CLI Info:**
```bash
.\target\release\systrix.exe info
```

### 2. Test New Features

#### ✅ Device Name (Hostname)
```bash
# CLI
.\target\release\systrix.exe info
# Look for "Device:" line

# TUI
.\target\release\systrix.exe tui
# Press 1 for Overview
# Check "Device:" in System Information
```

**Expected:**
```
Device: NOPALLL
OS: Windows 11 (26100)
Uptime: 8h 30m
```

#### 🔋 Battery Monitoring
```bash
.\target\release\systrix.exe tui
# Press 1 for Overview
# Battery gauge should appear (if laptop)
```

**Expected:**
```
┌─ Battery - Discharging ─────────────────────┐
│ 🔋 65% (1h 49m)                              │
└─────────────────────────────────────────────┘
```

#### ⚠️ Kill Process (Improved)
```bash
.\target\release\systrix.exe tui
# Press 2 for Processes
# Select a process with ↑↓
# Press k to kill
# Read the improved confirmation dialog
# Press Y to confirm or N to cancel
```

**Expected:**
- Beautiful confirmation dialog with full process info
- Clear warning messages
- Success/error feedback

### 3. Test Navigation Fixes

#### Settings Panel Navigation
```bash
.\target\release\systrix.exe tui
# Press 5 for Settings
# Press 1 → Should go to Overview (not Settings category)
# Press 5 → Back to Settings
# Press ↑↓ → Navigate categories in Settings
```

**Expected:**
- Number keys 1-5 always switch panels
- Arrow keys navigate within Settings

## 📋 Complete Feature Test Checklist

### Core Features
- [ ] TUI launches successfully
- [ ] All 5 panels accessible (1-5 keys)
- [ ] CPU, Memory, Disk, Network gauges working
- [ ] Process list displays correctly
- [ ] Arrow key navigation works

### New Features
- [ ] Device name shows in Overview
- [ ] Battery gauge appears (if laptop)
- [ ] Battery percentage is correct
- [ ] Kill confirmation dialog is beautiful
- [ ] Kill process actually works
- [ ] Success/error messages display

### Navigation
- [ ] Number keys 1-5 switch panels from anywhere
- [ ] Arrow keys work in Processes panel
- [ ] Arrow keys work in Settings panel
- [ ] PageUp/PageDown work
- [ ] Home/End work

### Process Management
- [ ] Search with `/` key works
- [ ] Process details with Enter key
- [ ] Kill with `k` key shows confirmation
- [ ] `Y` confirms kill
- [ ] `N` or ESC cancels kill
- [ ] Success message after kill
- [ ] Process list refreshes

### Themes
- [ ] `t` key cycles themes
- [ ] Dark theme works
- [ ] Light theme works
- [ ] Dracula theme works

### Other
- [ ] `p` key pauses/resumes
- [ ] `q` key quits
- [ ] Tab key cycles panels
- [ ] ESC closes modals

## 🎯 Recommended Test Sequence

### 1. Basic Test (2 minutes)
```bash
# Launch TUI
.\target\release\systrix.exe tui

# Quick navigation test
Press 1 → Overview (check device name & battery)
Press 2 → Processes
Press 3 → Network
Press 4 → Disk
Press 5 → Settings
Press q → Quit
```

### 2. Kill Process Test (3 minutes)
```bash
# Launch TUI
.\target\release\systrix.exe tui

# Open Notepad first
notepad

# In Systrix:
Press 2 → Processes
Press / → Search
Type "notepad"
Press Enter
Press k → Kill
Read confirmation dialog
Press Y → Confirm
Check success message
Press ESC → Close
Verify notepad closed
```

### 3. Battery Test (1 minute - laptop only)
```bash
.\target\release\systrix.exe tui
Press 1 → Overview
Check battery gauge
Unplug laptop → Should show discharging
Plug laptop → Should show charging ⚡
```

### 4. Navigation Test (2 minutes)
```bash
.\target\release\systrix.exe tui
Press 5 → Settings
Press ↑↓ → Navigate categories
Press 1 → Should go to Overview
Press 5 → Back to Settings
Press 2 → Should go to Processes
Press ↑↓ → Navigate process list
```

## 🐛 If Something Doesn't Work

### Battery Shows 0% or Unknown
**Possible causes:**
- Desktop PC (no battery)
- Battery not detected
- Parsing error

**Check:**
```bash
wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime /format:csv
```

### Kill Process Fails
**Possible causes:**
- Insufficient permissions
- System process
- Process already terminated

**Solution:**
- Run as Administrator
- Try different process
- Check error message

### Device Name Shows "Unknown"
**Possible causes:**
- Hostname not available
- System error

**Check:**
```bash
hostname
```

## 📊 Performance Check

After running for a few minutes:

```bash
# In another terminal
.\target\release\systrix.exe ps --sort cpu --limit 10

# Look for systrix.exe
# Should be around 3-5% CPU usage
```

## 🎉 Success Criteria

Your Systrix is working perfectly if:

✅ All panels display correctly
✅ Device name appears in Overview
✅ Battery shows correct percentage (if laptop)
✅ Kill process confirmation is beautiful
✅ Kill actually terminates processes
✅ Navigation works smoothly
✅ No crashes or errors
✅ CPU usage is low (3-5%)

## 📝 Next Steps

### Optional: Install to PATH

**Windows:**
```powershell
# Copy to a folder in PATH
copy .\target\release\systrix.exe C:\Windows\System32\

# Or add current folder to PATH (see INSTALLATION.md)
```

**Then you can run from anywhere:**
```bash
systrix tui
systrix info
systrix ps
```

### Optional: Create Desktop Shortcut

1. Right-click Desktop → New → Shortcut
2. Browse to `D:\local\systrix\target\release\systrix.exe`
3. Add argument: `tui`
4. Name it "Systrix Monitor"
5. Double-click to launch!

### Optional: Run on Startup

**Windows:**
1. Press Win+R
2. Type `shell:startup`
3. Create shortcut to systrix.exe with `tui` argument
4. Systrix will launch on boot!

## 🎊 Congratulations!

Systrix v0.2.0+ dengan fitur baru sudah siap digunakan:

- ✅ Device name monitoring
- ✅ Battery monitoring
- ✅ Improved kill confirmation
- ✅ Fixed navigation
- ✅ Better UX

**Enjoy monitoring your system! 🚀**

---

**Need Help?**
- Check KILL_PROCESS_GUIDE.md for kill feature details
- Check BATTERY_MONITORING.md for battery feature details
- Check README.md for general usage
- Open issue on GitHub if you find bugs
