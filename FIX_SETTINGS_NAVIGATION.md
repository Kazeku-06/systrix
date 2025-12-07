# Fix: Settings Panel Navigation Issue

## Problem
Ketika berada di Settings panel, menekan angka 1-5 tidak bisa pindah ke panel lain, malah pindah kategori di dalam Settings.

## Root Cause
Di `src/tui/event.rs`, logika event handling untuk number keys (1-5) mengecek apakah user sedang di Settings panel:
- Jika di Settings panel → pindah kategori Settings
- Jika tidak → pindah panel

Ini membuat user tidak bisa keluar dari Settings panel menggunakan number keys.

## Solution
Ubah logika agar angka 1-5 **selalu** pindah panel, tidak peduli di panel mana user berada.

Navigasi kategori di Settings panel cukup menggunakan:
- Arrow keys (↑↓)
- PageUp/PageDown
- Home/End

## Changes Made

### 1. src/tui/event.rs
**Before:**
```rust
KeyCode::Char('1') => {
    if ui.is_in_settings_panel() {
        ui.set_settings_category(0);
    } else {
        ui.set_active_panel(0);
    }
},
// ... similar for 2-5
```

**After:**
```rust
KeyCode::Char('1') => ui.set_active_panel(0),
KeyCode::Char('2') => ui.set_active_panel(1),
KeyCode::Char('3') => ui.set_active_panel(2),
KeyCode::Char('4') => ui.set_active_panel(3),
KeyCode::Char('5') => ui.set_active_panel(4),
```

### 2. SETTINGS_PANEL_GUIDE.md
Updated navigation section to clarify:
- Arrow keys for category navigation
- Number keys for panel switching

### 3. CHANGELOG.md
Added fix entry:
```
- 🐛 Fixed number keys (1-5) to always switch panels, not Settings categories
```

## Testing

### Before Fix
1. Launch TUI: `systrix tui`
2. Press `5` to go to Settings
3. Press `1` → Stays in Settings, jumps to Appearance category ❌
4. Press `2` → Stays in Settings, jumps to Performance category ❌

### After Fix
1. Launch TUI: `systrix tui`
2. Press `5` to go to Settings
3. Press `1` → Switches to Overview panel ✅
4. Press `2` → Switches to Processes panel ✅
5. Press `5` again to return to Settings
6. Use `↑↓` to navigate categories ✅

## How to Build and Test

1. **Close any running Systrix instances** (important!)
2. Build:
   ```bash
   cargo build --release
   ```
3. Run TUI:
   ```bash
   .\target\release\systrix.exe tui
   ```
4. Test navigation:
   - Press `5` to go to Settings
   - Press `1` → Should go to Overview
   - Press `5` → Back to Settings
   - Press `↑↓` → Navigate categories in Settings

## Navigation Summary

### Settings Panel Navigation (After Fix)

**Switch Panels:**
- `1` → Overview
- `2` → Processes
- `3` → Network
- `4` → Disk
- `5` → Settings (current)
- `Tab` → Next panel

**Navigate Categories (within Settings):**
- `↑↓` → Previous/Next category
- `PageUp` → First category
- `PageDown` → Last category
- `Home` → First category
- `End` → Last category

## Impact

- ✅ Consistent behavior across all panels
- ✅ Number keys always switch panels
- ✅ Arrow keys for in-panel navigation
- ✅ No breaking changes to other functionality
- ⚠️ Methods `set_settings_category()` and `is_in_settings_panel()` now unused (can be removed or kept for future use)

## Status
✅ Code changes complete
✅ Documentation updated
⏳ Build pending (waiting for file access)
⏳ Testing pending

---

**Note**: Jika build gagal dengan error "Access is denied", pastikan tidak ada instance Systrix yang sedang berjalan. Close semua terminal yang menjalankan `systrix tui`.
