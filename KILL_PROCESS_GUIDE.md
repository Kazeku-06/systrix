# Kill Process Feature - User Guide

## ✅ Improved Kill Process Confirmation

Kill process feature sekarang memiliki confirmation dialog yang lebih informatif dan aman!

## 🎯 How to Kill a Process

### Step-by-Step Guide

1. **Launch TUI**
   ```bash
   .\target\release\systrix.exe tui
   ```

2. **Go to Processes Panel**
   - Press `2` or navigate with Tab

3. **Select a Process**
   - Use `↑↓` arrow keys to navigate
   - Or use `/` to search for specific process

4. **Initiate Kill**
   - Press `k` key

5. **Confirmation Dialog Appears**
   ```
   ╔════════════════════════════════════════════════╗
   ║         ⚠️  KILL PROCESS CONFIRMATION          ║
   ╚════════════════════════════════════════════════╝
   
   You are about to terminate:
   
   📋 Process Name:  chrome.exe
   🔢 Process ID:    12345
   👤 User:          YourUser
   💻 CPU Usage:     15.5%
   💾 Memory:        25.3%
   📁 Path:          C:\Program Files\...\chrome.exe
   
   ⚠️  WARNING: This action cannot be undone!
   ⚠️  The process will be forcefully terminated.
   
   ┌──────────────────────────────────────────────┐
   │  Press [Y] to KILL the process               │
   │  Press [N] or [ESC] to CANCEL                │
   └──────────────────────────────────────────────┘
   ```

6. **Confirm or Cancel**
   - Press `Y` to kill the process
   - Press `N` or `ESC` to cancel

7. **Result Message**
   
   **Success:**
   ```
   ╔════════════════════════════════════════════════╗
   ║              ✅ SUCCESS                        ║
   ╚════════════════════════════════════════════════╝
   
   Process 12345 has been terminated successfully!
   
   The process has been forcefully killed and
   removed from the system.
   
   ┌──────────────────────────────────────────────┐
   │  Press [ESC] to close this message           │
   └──────────────────────────────────────────────┘
   ```
   
   **Error:**
   ```
   ╔════════════════════════════════════════════════╗
   ║              ❌ ERROR                          ║
   ╚════════════════════════════════════════════════╝
   
   Failed to kill process 12345!
   
   Error Details:
   Access denied
   
   Possible reasons:
   • Insufficient permissions (try running as admin)
   • Process already terminated
   • System/protected process
   
   ┌──────────────────────────────────────────────┐
   │  Press [ESC] to close this message           │
   └──────────────────────────────────────────────┘
   ```

## 🔧 Technical Details

### Windows Implementation
```rust
// Uses taskkill command with /F (force) flag
taskkill /PID <pid> /F
```

### Linux/macOS Implementation
```rust
// Uses kill command with -9 (SIGKILL) signal
kill -9 <pid>
```

### Safety Features

1. **Confirmation Required**
   - Always shows confirmation dialog
   - Displays full process information
   - Clear warning about consequences

2. **Detailed Information**
   - Process name and PID
   - User ownership
   - Resource usage
   - Executable path

3. **Error Handling**
   - Graceful error messages
   - Helpful troubleshooting hints
   - No crashes on failure

4. **Visual Feedback**
   - Color-coded modals (Red for warning, Cyan for info)
   - Clear success/error indicators
   - Emoji icons for better UX

## ⚠️ Important Notes

### Permissions

**Windows:**
- Some processes require Administrator privileges
- Right-click and "Run as Administrator" if needed
- System processes are protected

**Linux/macOS:**
- May need sudo for processes owned by other users
- Root processes require root access
- Use with caution!

### Protected Processes

Cannot kill:
- System critical processes (csrss.exe, winlogon.exe, etc.)
- Processes owned by SYSTEM (without admin)
- Kernel processes
- Protected services

### Best Practices

1. **Always read the confirmation dialog carefully**
2. **Check the process name and PID**
3. **Save your work before killing processes**
4. **Don't kill system processes unless you know what you're doing**
5. **Use regular termination first (close window) before force kill**

## 🎨 UI Features

### Confirmation Dialog
- **Width**: 70% of screen
- **Height**: 70% of screen
- **Border Color**: Red (warning)
- **Background**: Black
- **Text Color**: White

### Success Dialog
- **Border Color**: Cyan (info)
- **Icon**: ✅
- **Auto-refresh**: Process list updates after kill

### Error Dialog
- **Border Color**: Red (error)
- **Icon**: ❌
- **Helpful hints**: Troubleshooting suggestions

## 🔍 Troubleshooting

### "Access Denied" Error

**Solution:**
1. Close Systrix
2. Right-click systrix.exe
3. Select "Run as Administrator"
4. Try killing the process again

### "Process Not Found" Error

**Reason:**
- Process already terminated
- Process ID changed

**Solution:**
- Press ESC to close error
- Process list will refresh
- Select another process

### Kill Doesn't Work

**Check:**
1. Are you running as Administrator? (Windows)
2. Is it a system process?
3. Is the process frozen/hung?
4. Try restarting Systrix

## 📊 Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `k` | Kill selected process (shows confirmation) |
| `Y` | Confirm kill (in confirmation dialog) |
| `N` | Cancel kill (in confirmation dialog) |
| `ESC` | Cancel/Close dialog |
| `↑↓` | Navigate process list |
| `/` | Search processes |

## 🎯 Use Cases

### 1. Unresponsive Application
```
1. Press 2 → Processes panel
2. Press / → Search
3. Type app name
4. Press k → Kill
5. Press Y → Confirm
```

### 2. High CPU Process
```
1. Processes already sorted by CPU
2. Select top process
3. Press k → Kill
4. Press Y → Confirm
```

### 3. Memory Leak
```
1. Sort by memory (already sorted)
2. Find suspicious process
3. Press k → Kill
4. Press Y → Confirm
```

## ✅ Testing Checklist

After build completes, test these scenarios:

- [ ] Kill a normal process (e.g., notepad)
- [ ] Try to kill a system process (should fail gracefully)
- [ ] Cancel kill with N key
- [ ] Cancel kill with ESC key
- [ ] Confirm kill with Y key
- [ ] Check success message appears
- [ ] Check process list refreshes
- [ ] Try killing non-existent PID (error handling)

## 🚀 What's Next

After build completes:

```bash
# Run TUI
.\target\release\systrix.exe tui

# Test kill feature
1. Press 2 (Processes)
2. Select a safe process (e.g., notepad)
3. Press k
4. Read confirmation dialog
5. Press Y to confirm
6. Check success message
```

---

**⚠️ Remember: With great power comes great responsibility!**

Always be careful when killing processes. Make sure you know what the process does before terminating it.
