# Systrix Examples

## Running Systrix

The examples below use `systrix` as the command. Depending on your setup:

- **If installed to PATH**: `systrix <command>`
- **Linux/macOS (from project dir)**: `./target/release/systrix <command>`
- **Windows (from project dir)**: `.\target\release\systrix.exe <command>`

## CLI Examples

### System Information

**Linux/macOS:**
```bash
$ ./target/release/systrix info
```

**Windows:**
```powershell
PS> .\target\release\systrix.exe info
```

**Output:**
```
╔══════════════════════════════════════════════════════════╗
║                    SYSTRIX - System Info                 ║
╚══════════════════════════════════════════════════════════╝

CPU:
  Model: Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz
  Cores: 6 physical, 12 logical
  Usage: 23.4%
  Frequency: 2600 MHz

Memory:
  Total: 16.0 GB
  Used: 8.2 GB (51.3%)
  Available: 7.8 GB

Disk:
  Total: 512.0 GB
  Used: 312.5 GB (61.0%)
  Available: 199.5 GB

System:
  OS: Linux 6.1.0
  Uptime: 2d 5h 32m
```

### Process List

**Command:**
```bash
# Linux/macOS
$ ./target/release/systrix ps --limit 10 --sort cpu

# Windows
PS> .\target\release\systrix.exe ps --limit 10 --sort cpu
```

**Output:**
```
PID      USER       NAME                  CPU%   MEM%   IO_R     IO_W     THREADS
1234     user       chrome                34.2   5.1    2.1 MB   0.3 MB        42
2345     user       code                  12.0   3.2    1.5 MB   0.8 MB        28
3456     user       firefox               8.5    4.3    0.9 MB   0.2 MB        35
4567     root       systemd               2.1    0.1    0.0 B    0.0 B          1
5678     user       terminal              1.2    0.8    0.0 B    0.0 B          4
6789     user       systrix               0.8    0.2    0.0 B    0.0 B          1

Showing 10 of 156 processes
```

### Filter Processes

**Command:**
```bash
# Linux/macOS
$ ./target/release/systrix ps --filter chrome --limit 5

# Windows
PS> .\target\release\systrix.exe ps --filter chrome --limit 5
```

**Output:**
```
PID      USER       NAME                  CPU%   MEM%   IO_R     IO_W     THREADS
1234     user       chrome                34.2   5.1    2.1 MB   0.3 MB        42
1235     user       chrome                12.1   2.3    0.5 MB   0.1 MB        18
1236     user       chrome                8.4    1.9    0.3 MB   0.0 B         12

Showing 3 of 3 processes
```

### Kill Process

**Interactive (with confirmation):**
```bash
# Linux/macOS
$ ./target/release/systrix kill 1234

# Windows
PS> .\target\release\systrix.exe kill 1234
```

**Output:**
```
About to kill process 1234 with signal SIGTERM
Continue? (y/N): y
✓ Process 1234 killed successfully
```

**Force kill without confirmation:**
```bash
# Linux/macOS
$ ./target/release/systrix kill 1234 --signal SIGKILL --force

# Windows
PS> .\target\release\systrix.exe kill 1234 --signal SIGKILL --force
```

**Output:**
```
✓ Process 1234 killed successfully
```

**Attempting to kill system process:**
```bash
# Linux/macOS
$ ./target/release/systrix kill 1

# Windows
PS> .\target\release\systrix.exe kill 1
```

**Output:**
```
✗ Cannot kill system process (PID 1). Use --force to override (not recommended)
```

### Network Interfaces

**Command:**
```bash
# Linux/macOS
$ ./target/release/systrix net

# Windows
PS> .\target\release\systrix.exe net
```

**Output:**
```
Network Interfaces:
INTERFACE       RX_BYTES     TX_BYTES      RX_RATE      TX_RATE
eth0            1.2 GB       456.7 MB      1.2 MB/s     0.3 MB/s
wlan0           0 B          0 B           0 B/s        0 B/s
lo              45.6 MB      45.6 MB       0 B/s        0 B/s
```

### Disk Partitions

**Command:**
```bash
# Linux/macOS
$ ./target/release/systrix disk

# Windows
PS> .\target\release\systrix.exe disk
```

**Output:**
```
Disk Partitions:
MOUNT                TYPE           TOTAL        USED    AVAILABLE     USE%
/                    ext4         512.0 GB    312.5 GB    199.5 GB    61.0%
/home                ext4         1.0 TB      456.8 GB    543.2 GB    45.7%
/boot                vfat         512.0 MB    128.3 MB    383.7 MB    25.1%
```

### Export Report

**Command:**
```bash
# Linux/macOS
$ ./target/release/systrix report --output system-report.json

# Windows
PS> .\target\release\systrix.exe report --output system-report.json
```

**Output:**
```
✓ Report exported to: system-report.json
```

Example JSON output:
```json
{
  "timestamp": "2025-12-05T10:30:45Z",
  "cpu": {
    "model": "Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz",
    "physical_cores": 6,
    "logical_cores": 12,
    "global_usage": 23.4,
    "frequency": 2600.0,
    "uptime": 183120,
    "os_name": "Linux 6.1.0"
  },
  "memory": {
    "total": 17179869184,
    "used": 8813084672,
    "available": 8366784512,
    "usage_percent": 51.3
  },
  "processes": [
    {
      "pid": 1234,
      "name": "chrome",
      "user": "user",
      "cpu_usage": 34.2,
      "memory_usage": 5.1,
      "threads": 42
    }
  ]
}
```

## TUI Examples

### Launch TUI

**Linux/macOS:**
```bash
$ ./target/release/systrix tui
```

**Windows:**
```powershell
PS> .\target\release\systrix.exe tui
```

### Custom Refresh Rate

**Linux/macOS:**
```bash
$ ./target/release/systrix tui --refresh-interval 1000
```

**Windows:**
```powershell
PS> .\target\release\systrix.exe tui --refresh-interval 1000
```

### TUI Layout (ASCII Art)

```
┌────────────────────────────────────────────────────────────────────────┐
│ SYSTRIX │ CPU:  23.4% │ RAM:  51.3% │ DISK:  61.0%                     │
└────────────────────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────────────────┐
│ [Overview] [Processes] [Network] [Disk] [Settings]                     │
└────────────────────────────────────────────────────────────────────────┘
┌─ CPU Usage ────────────────────────────────────────────────────────────┐
│ ▇▇▇▇▇▇▇▇▇▇▇▇░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 23.4%   │
└────────────────────────────────────────────────────────────────────────┘
┌─ Memory Usage ─────────────────────────────────────────────────────────┐
│ ▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 51.3%     │
│ (8.2 GB / 16.0 GB)                                                     │
└────────────────────────────────────────────────────────────────────────┘
┌─ Disk Usage ───────────────────────────────────────────────────────────┐
│ ▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇░░░░░░░░░░░░░░░░░░░░░░░░░░░ 61.0%     │
│ (312.5 GB / 512.0 GB)                                                  │
└────────────────────────────────────────────────────────────────────────┘
┌─ Network ──────────────────────────────────────────────────────────────┐
│ ↓ 1.2 GB | ↑ 456.7 MB                                                  │
└────────────────────────────────────────────────────────────────────────┘
┌─ System Information ───────────────────────────────────────────────────┐
│ CPU Model: Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz                   │
│ Cores: 6 physical, 12 logical                                          │
│ Frequency: 2600 MHz                                                    │
│ Uptime: 2d 5h 32m                                                      │
│ OS: Linux 6.1.0                                                        │
└────────────────────────────────────────────────────────────────────────┘
 [q]Quit [1-5]Panels [↑↓]Navigate [k]Kill [p]Pause [t]Theme
```

### Process Panel

```
┌─ Processes (156 total) ────────────────────────────────────────────────┐
│ PID      USER       NAME                CPU%   MEM%   THREADS          │
│ 1234     user       chrome              34.2   5.1        42           │
│ 2345     user       code                12.0   3.2        28           │
│ 3456     user       firefox              8.5   4.3        35           │
│ 4567     root       systemd              2.1   0.1         1           │
│ 5678     user       terminal             1.2   0.8         4           │
│ 6789     user       systrix              0.8   0.2         1           │
│                                                                         │
│ [↑↓ select]  [k Kill] [s Suspend] [r Resume] [Enter details]          │
└────────────────────────────────────────────────────────────────────────┘
```

### Kill Confirmation Modal

```
┌─ Confirmation ─────────────────────────────────────────────────────────┐
│                                                                         │
│  Kill process chrome (PID 1234)?                                       │
│  Press 'y' to confirm, ESC to cancel                                   │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

## Advanced Usage

### Monitoring Specific Process

**Linux/macOS:**
```bash
# Watch a specific process (updates every second)
watch -n 1 './target/release/systrix ps --filter myapp --limit 1'
```

**Windows (PowerShell):**
```powershell
# Watch a specific process (updates every second)
while ($true) { 
    Clear-Host
    .\target\release\systrix.exe ps --filter myapp --limit 1
    Start-Sleep -Seconds 1
}
```

### Automated Reporting

**Linux/macOS (cron):**
```bash
# Add to crontab (crontab -e)
# Generate hourly reports
0 * * * * /usr/local/bin/systrix report --output /var/log/systrix/report-$(date +\%Y\%m\%d-\%H).json
```

**Windows (Task Scheduler):**
```powershell
# Create scheduled task (run as Administrator)
$action = New-ScheduledTaskAction -Execute "C:\path\to\systrix.exe" -Argument "report --output C:\logs\report-$(Get-Date -Format 'yyyyMMdd-HH').json"
$trigger = New-ScheduledTaskTrigger -Once -At (Get-Date) -RepetitionInterval (New-TimeSpan -Hours 1)
Register-ScheduledTask -TaskName "Systrix Report" -Action $action -Trigger $trigger
```

### Process Management Script

**Linux/macOS (bash):**
```bash
#!/bin/bash
# Kill all Chrome processes
./target/release/systrix ps --filter chrome | tail -n +2 | awk '{print $1}' | while read pid; do
    ./target/release/systrix kill $pid --force
done
```

**Windows (PowerShell):**
```powershell
# Kill all Chrome processes
.\target\release\systrix.exe ps --filter chrome | 
    Select-Object -Skip 2 | 
    ForEach-Object {
        $pid = ($_ -split '\s+')[0]
        .\target\release\systrix.exe kill $pid --force
    }
```

### Remote Monitoring (with remote feature)

```bash
# Start remote agent
systrix remote --port 8080 --bind 0.0.0.0

# Query from another machine
curl http://server:8080/metrics
```
