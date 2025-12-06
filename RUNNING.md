# Running Systrix - Complete Guide

## Quick Start

After building with `cargo build --release`, the binary will be at:

- **Linux/macOS**: `./target/release/systrix`
- **Windows**: `.\target\release\systrix.exe`

## Installation to System PATH

### Linux/macOS

```bash
# Option 1: Copy to /usr/local/bin (requires sudo)
sudo cp target/release/systrix /usr/local/bin/
systrix version

# Option 2: Copy to ~/.local/bin (user-only, no sudo)
mkdir -p ~/.local/bin
cp target/release/systrix ~/.local/bin/
# Add to PATH if not already (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
systrix version

# Option 3: Create symlink
sudo ln -s $(pwd)/target/release/systrix /usr/local/bin/systrix
systrix version
```

### Windows

**Option 1: Add to System PATH (Permanent)**
```powershell
# Run PowerShell as Administrator
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
$newPath = "$currentPath;D:\local\systrix\target\release"
[Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")

# Restart terminal, then:
systrix version
```

**Option 2: Add to User PATH (No Admin)**
```powershell
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
$newPath = "$currentPath;D:\local\systrix\target\release"
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")

# Restart terminal, then:
systrix version
```

**Option 3: PowerShell Alias (Session-only)**
```powershell
Set-Alias systrix "D:\local\systrix\target\release\systrix.exe"
systrix version

# To make permanent, add to PowerShell profile:
notepad $PROFILE
# Add this line:
# Set-Alias systrix "D:\local\systrix\target\release\systrix.exe"
```

## Running Commands

### From Project Directory (Without Installation)

**Linux/macOS:**
```bash
./target/release/systrix info
./target/release/systrix ps --limit 10
./target/release/systrix tui
```

**Windows:**
```powershell
.\target\release\systrix.exe info
.\target\release\systrix.exe ps --limit 10
.\target\release\systrix.exe tui
```

### After Installation to PATH

```bash
# Works on all platforms
systrix info
systrix ps --limit 10
systrix tui
```

## Common Commands

### System Information
```bash
systrix info
```

### Process Monitoring
```bash
# List top 10 processes by CPU
systrix ps --limit 10

# Sort by memory
systrix ps --sort mem --limit 10

# Filter by name
systrix ps --filter chrome

# Sort by disk I/O
systrix ps --sort io --limit 10
```

### Process Management
```bash
# Kill process (with confirmation)
systrix kill 1234

# Kill without confirmation
systrix kill 1234 --force

# Kill with specific signal
systrix kill 1234 --signal SIGKILL
```

### Network & Disk
```bash
# Network interfaces
systrix net

# Disk partitions
systrix disk
```

### Export & Reporting
```bash
# Export to JSON
systrix report --output report.json

# View report
cat report.json | jq .        # Linux/macOS
type report.json               # Windows
```

### Interactive TUI
```bash
# Launch TUI
systrix tui

# Custom refresh interval (milliseconds)
systrix tui --refresh-interval 1000

# Minimum refresh is 100ms
systrix tui --refresh-interval 100
```

## TUI Keyboard Shortcuts

Once in TUI mode:

### Navigation
- `q` - Quit application
- `Ctrl+C` - Force quit
- `1` - Overview panel
- `2` - Processes panel
- `3` - Network panel
- `4` - Disk panel
- `5` - Settings panel
- `Tab` - Next panel
- `↑` / `↓` - Navigate list
- `PageUp` / `PageDown` - Scroll by page
- `Home` - Jump to top
- `End` - Jump to bottom

### Actions
- `Enter` - Show details (TODO)
- `k` - Kill selected process
- `s` - Suspend process (Linux only)
- `r` - Resume process (Linux only)
- `p` - Pause/resume refresh
- `t` - Toggle theme (Dark → Light → Dracula)
- `/` - Search (TODO)
- `Esc` - Cancel action/close modal

## Running with Elevated Privileges

Some operations require elevated privileges:

### Linux/macOS
```bash
# Run as root
sudo systrix tui

# Or for specific command
sudo systrix kill 1234
```

### Windows
```powershell
# Run PowerShell as Administrator, then:
systrix tui

# Or right-click PowerShell → "Run as Administrator"
```

## Running in Background

### Linux/macOS
```bash
# Run in background
systrix tui &

# Run with nohup (survives terminal close)
nohup systrix tui > /dev/null 2>&1 &

# View background jobs
jobs

# Bring to foreground
fg %1

# Kill background job
kill %1
```

### Windows
```powershell
# Run in new window
Start-Process systrix.exe -ArgumentList "tui"

# Run minimized
Start-Process systrix.exe -ArgumentList "tui" -WindowStyle Minimized
```

## Automated Execution

### Linux/macOS (cron)

```bash
# Edit crontab
crontab -e

# Add entries:
# Run every hour
0 * * * * /usr/local/bin/systrix report --output /var/log/systrix/report-$(date +\%Y\%m\%d-\%H).json

# Run every 5 minutes
*/5 * * * * /usr/local/bin/systrix ps --limit 5 >> /var/log/systrix/processes.log
```

### Windows (Task Scheduler)

**Via GUI:**
1. Open Task Scheduler
2. Create Basic Task
3. Set trigger (e.g., hourly)
4. Action: Start a program
5. Program: `C:\path\to\systrix.exe`
6. Arguments: `report --output C:\logs\report.json`

**Via PowerShell:**
```powershell
# Create hourly report task
$action = New-ScheduledTaskAction -Execute "C:\path\to\systrix.exe" -Argument "report --output C:\logs\report.json"
$trigger = New-ScheduledTaskTrigger -Once -At (Get-Date) -RepetitionInterval (New-TimeSpan -Hours 1)
Register-ScheduledTask -TaskName "Systrix Hourly Report" -Action $action -Trigger $trigger
```

## Troubleshooting

### Command Not Found

**Linux/macOS:**
```bash
# Check if in PATH
which systrix

# If not, use full path
/full/path/to/systrix info

# Or add to PATH
export PATH="/path/to/systrix/target/release:$PATH"
```

**Windows:**
```powershell
# Check if in PATH
where.exe systrix

# If not, use full path
C:\full\path\to\systrix.exe info

# Or add to PATH (see Installation section above)
```

### Permission Denied

**Linux/macOS:**
```bash
# Make executable
chmod +x target/release/systrix

# Or run with sudo
sudo ./target/release/systrix kill 1234
```

**Windows:**
```powershell
# Run as Administrator
# Right-click PowerShell → "Run as Administrator"
```

### Terminal Issues

**TUI not displaying correctly:**
- Ensure terminal supports ANSI colors
- Minimum terminal size: 80x24
- Try different terminal:
  - Linux: gnome-terminal, konsole, alacritty
  - macOS: iTerm2, Terminal.app
  - Windows: Windows Terminal, ConEmu

**Terminal not restored after crash:**
```bash
# Linux/macOS
reset

# Or
stty sane
```

```powershell
# Windows - restart terminal
```

## Performance Tips

### Reduce CPU Usage
```bash
# Increase refresh interval
systrix tui --refresh-interval 2000

# Use CLI instead of TUI for scripting
systrix ps --limit 10
```

### Reduce Memory Usage
```bash
# Limit process list
systrix ps --limit 20

# Use specific commands instead of TUI
systrix info
```

## Integration Examples

### Shell Script (Linux/macOS)
```bash
#!/bin/bash
# monitor.sh - Simple monitoring script

while true; do
    clear
    echo "=== System Monitor ==="
    ./target/release/systrix info
    echo ""
    echo "=== Top Processes ==="
    ./target/release/systrix ps --limit 5
    sleep 5
done
```

### PowerShell Script (Windows)
```powershell
# monitor.ps1 - Simple monitoring script

while ($true) {
    Clear-Host
    Write-Host "=== System Monitor ==="
    .\target\release\systrix.exe info
    Write-Host ""
    Write-Host "=== Top Processes ==="
    .\target\release\systrix.exe ps --limit 5
    Start-Sleep -Seconds 5
}
```

### Python Integration
```python
import subprocess
import json

# Run systrix and get JSON output
result = subprocess.run(
    ['./target/release/systrix', 'report', '--output', 'report.json'],
    capture_output=True
)

# Parse JSON
with open('report.json') as f:
    data = json.load(f)
    
print(f"CPU Usage: {data['cpu']['global_usage']}%")
print(f"Memory Usage: {data['memory']['usage_percent']}%")
```

## Next Steps

- Read [EXAMPLES.md](EXAMPLES.md) for more usage examples
- Read [QUICKSTART.md](QUICKSTART.md) for quick reference
- Read [README.md](README.md) for full documentation

---

**Happy Monitoring! 🚀**
