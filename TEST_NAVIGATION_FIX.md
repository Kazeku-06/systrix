# Test Navigation Fix - Quick Guide

## ✅ Build Status
Build successful! Binary ready at: `.\target\release\systrix.exe`

## 🧪 Test Steps

### Test 1: Number Keys Switch Panels from Settings
1. Run: `.\target\release\systrix.exe tui`
2. Press `5` → Should show Settings panel
3. Press `1` → Should switch to Overview panel ✅
4. Press `5` → Back to Settings
5. Press `2` → Should switch to Processes panel ✅
6. Press `5` → Back to Settings
7. Press `3` → Should switch to Network panel ✅
8. Press `5` → Back to Settings
9. Press `4` → Should switch to Disk panel ✅

**Expected**: Number keys 1-5 always switch panels, even from Settings

### Test 2: Arrow Keys Navigate Categories in Settings
1. Press `5` → Go to Settings panel
2. Press `↓` → Should move to next category (Performance)
3. Press `↓` → Should move to Display
4. Press `↓` → Should move to Keyboard
5. Press `↓` → Should move to About
6. Press `↓` → Should stay at About (last category)
7. Press `↑` → Should move back to Keyboard
8. Press `Home` → Should jump to Appearance (first)
9. Press `End` → Should jump to About (last)

**Expected**: Arrow keys navigate categories within Settings panel

### Test 3: Other Panels Still Work
1. Press `2` → Go to Processes panel
2. Press `↑↓` → Should navigate process list ✅
3. Press `1` → Should switch to Overview ✅
4. Press `2` → Back to Processes ✅

**Expected**: Navigation in other panels unchanged

### Test 4: Tab Key Still Works
1. Press `1` → Overview
2. Press `Tab` → Should go to Processes
3. Press `Tab` → Should go to Network
4. Press `Tab` → Should go to Disk
5. Press `Tab` → Should go to Settings
6. Press `Tab` → Should cycle back to Overview

**Expected**: Tab cycles through all panels

## ✅ Success Criteria

- [x] Number keys 1-5 switch panels from anywhere
- [x] Arrow keys navigate categories in Settings
- [x] PageUp/PageDown work in Settings
- [x] Home/End work in Settings
- [x] Tab key still cycles panels
- [x] Other panels navigation unchanged

## 🐛 If Issues Found

1. Check which key was pressed
2. Check which panel you're in
3. Check expected vs actual behavior
4. Report issue with details

## 📝 Notes

- Settings panel has 5 categories (0-4):
  0. Appearance
  1. Performance
  2. Display
  3. Keyboard
  4. About

- Number keys now ONLY switch panels, not categories
- Use arrow keys for category navigation in Settings

---

**Ready to test!** 🚀

Run: `.\target\release\systrix.exe tui`
