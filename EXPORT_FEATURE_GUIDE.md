# Export Data Feature - User Guide

## ✅ New Feature: Data Export

Systrix sekarang dapat mengexport data sistem ke file CSV atau JSON!

## 🎯 How to Export Data

### 1. CLI Export

**Basic Export (JSON):**
```bash
.\target\release\systrix.exe export
```

**Export to CSV:**
```bash
.\target\release\systrix.exe export --format csv
```

**Custom filename:**
```bash
.\target\release\systrix.exe export --format json --output my_report.json
```

**Export without processes:**
```bash
.\target\release\systrix.exe export --processes false
```

**Limit processes:**
```bash
.\target\release\systrix.exe export --process-limit 50
```

### 2. TUI Export

**In TUI mode:**
```bash
.\target\release\systrix.exe tui

# Export to JSON
Press e

# Export to CSV
Press Ctrl+C
```

**Success Modal:**
```
╔════════════════════════════════════════════════╗
║              ✅ EXPORT SUCCESS                 ║
╚════════════════════════════════════════════════╝

Data exported successfully!

📁 File: systrix_export_20251207_143022.json
📊 Format: JSON

The file contains:
• System information
• CPU, Memory, Disk, Network stats
• Battery status (if available)
• Process list (156 processes)

┌──────────────────────────────────────────────┐
│  Press [ESC] to close this message           │
└──────────────────────────────────────────────┘
```

## 📊 Export Formats

### 1. JSON Format

**Structure:**
```json
{
  "timestamp": "2025-12-07 14:30:22",
  "system": {
    "device": "NOPALLL",
    "os": "Windows 11 (26100)",
    "uptime_seconds": 28845
  },
  "cpu": {
    "model": "AMD Ryzen 5 3500U with Radeon Vega Mobile Gfx",
    "physical_cores": 4,
    "logical_cores": 8,
    "usage_percent": 15.2,
    "frequency_mhz": 2100.0,
    "per_core_usage": [12.5, 18.3, 14.1, 16.8, ...]
  },
  "memory": {
    "total_bytes": 8254464000,
    "used_bytes": 7168512000,
    "available_bytes": 1085952000,
    "usage_percent": 86.8
  },
  "disk": {
    "total_bytes": 255066636288,
    "used_bytes": 173423616000,
    "available_bytes": 81643020288,
    "usage_percent": 68.0
  },
  "disk_partitions": [
    {
      "name": "C:",
      "mount_point": "C:\\",
      "filesystem": "NTFS",
      "total_bytes": 255066636288,
      "used_bytes": 173423616000,
      "available_bytes": 81643020288,
      "usage_percent": 68.0
    }
  ],
  "network": {
    "total_rx_bytes": 1234567890,
    "total_tx_bytes": 987654321,
    "interfaces": [
      {
        "name": "Wi-Fi",
        "received_bytes": 1234567890,
        "transmitted_bytes": 987654321,
        "rx_rate_bytes_per_sec": 1024,
        "tx_rate_bytes_per_sec": 512,
        "packets_received": 123456,
        "packets_transmitted": 98765
      }
    ]
  },
  "battery": {
    "percentage": 65.0,
    "status": "Discharging",
    "is_charging": false,
    "is_plugged": false,
    "time_remaining_seconds": 6540,
    "health_percent": 95.0,
    "technology": "Li-ion",
    "vendor": "Unknown"
  },
  "processes": [
    {
      "pid": 1234,
      "name": "chrome.exe",
      "user": "YourUser",
      "cpu_percent": 15.5,
      "memory_percent": 25.3,
      "disk_read_bytes": 1048576,
      "disk_write_bytes": 524288,
      "threads": 32,
      "status": "Running",
      "executable": "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe"
    }
  ]
}
```

### 2. CSV Format

**Structure:**
```csv
Systrix System Monitor Export
Timestamp,2025-12-07 14:30:22

=== SYSTEM INFORMATION ===
Device,NOPALLL
OS,Windows 11 (26100)
Uptime (seconds),28845

=== CPU ===
Model,AMD Ryzen 5 3500U with Radeon Vega Mobile Gfx
Physical Cores,4
Logical Cores,8
Usage (%),15.20
Frequency (MHz),2100

=== MEMORY ===
Total (bytes),8254464000
Used (bytes),7168512000
Available (bytes),1085952000
Usage (%),86.80

=== DISK (Total) ===
Total (bytes),255066636288
Used (bytes),173423616000
Available (bytes),81643020288
Usage (%),68.00

=== DISK PARTITIONS ===
Name,Mount Point,Filesystem,Total (bytes),Used (bytes),Available (bytes),Usage (%)
C:,C:\,NTFS,255066636288,173423616000,81643020288,68.00

=== NETWORK ===
Total RX (bytes),1234567890
Total TX (bytes),987654321

=== NETWORK INTERFACES ===
Name,RX (bytes),TX (bytes),RX Rate (bytes/s),TX Rate (bytes/s),Packets RX,Packets TX
Wi-Fi,1234567890,987654321,1024,512,123456,98765

=== BATTERY ===
Percentage,65
Status,Discharging
Charging,false
Plugged,false
Time Remaining (seconds),6540
Health (%),95

=== PROCESSES ===
PID,Name,User,CPU (%),Memory (%),Disk Read (bytes),Disk Write (bytes),Threads,Status,Executable
1234,chrome.exe,YourUser,15.50,25.30,1048576,524288,32,Running,C:\Program Files\Google\Chrome\Application\chrome.exe
```

## 🔧 CLI Options

### Export Command

```bash
systrix export [OPTIONS]
```

**Options:**
- `--format <FORMAT>` - Export format: csv, json (default: json)
- `--output <FILE>` - Output file path (optional, auto-generated if not provided)
- `--processes <BOOL>` - Include process list (default: true)
- `--process-limit <NUM>` - Maximum number of processes (default: 100)

**Examples:**

```bash
# Basic JSON export
systrix export

# CSV export with custom filename
systrix export --format csv --output system_report.csv

# JSON without processes
systrix export --format json --processes false --output system_only.json

# Export with limited processes
systrix export --format csv --process-limit 50 --output top50_processes.csv
```

### Legacy Report Command

```bash
systrix report [--output <FILE>]
```

**Note:** Report command now uses the new export functionality internally.

## ⌨️ TUI Shortcuts

| Key | Action |
|-----|--------|
| `e` | Export to JSON |
| `Ctrl+C` | Export to CSV |
| `ESC` | Close export modal |

## 📁 File Naming

### Auto-generated Names

**Format:** `systrix_export_YYYYMMDD_HHMMSS.{ext}`

**Examples:**
- `systrix_export_20251207_143022.json`
- `systrix_export_20251207_143022.csv`

### Custom Names

You can specify custom filenames:
```bash
systrix export --output my_report.json
systrix export --format csv --output server_stats.csv
```

## 📊 Data Included

### Always Included
- ✅ System information (device name, OS, uptime)
- ✅ CPU stats (model, cores, usage, frequency)
- ✅ Memory stats (total, used, available, percentage)
- ✅ Disk stats (total usage + partition details)
- ✅ Network stats (total + per-interface)
- ✅ Timestamp

### Conditionally Included
- 🔋 Battery stats (if laptop/battery present)
- 📋 Process list (if --processes=true)

### Configurable
- 🔢 Number of processes (--process-limit)
- 📄 Output format (CSV vs JSON)
- 📁 Output filename

## 🎯 Use Cases

### 1. System Reports
```bash
# Generate daily system report
systrix export --format csv --output daily_report_$(date +%Y%m%d).csv
```

### 2. Performance Analysis
```bash
# Export with top 20 processes
systrix export --process-limit 20 --output performance_analysis.json
```

### 3. System Documentation
```bash
# System info only (no processes)
systrix export --processes false --output system_specs.json
```

### 4. Monitoring Integration
```bash
# JSON for automated processing
systrix export --format json --output monitoring_data.json
```

### 5. Troubleshooting
```bash
# Full system snapshot with all processes
systrix export --process-limit 500 --output troubleshoot_$(hostname).json
```

## 🔍 Data Analysis

### JSON Processing

**Python example:**
```python
import json

with open('systrix_export_20251207_143022.json', 'r') as f:
    data = json.load(f)

print(f"CPU Usage: {data['cpu']['usage_percent']}%")
print(f"Memory Usage: {data['memory']['usage_percent']}%")
print(f"Top Process: {data['processes'][0]['name']}")
```

**PowerShell example:**
```powershell
$data = Get-Content 'systrix_export_20251207_143022.json' | ConvertFrom-Json
Write-Host "CPU Usage: $($data.cpu.usage_percent)%"
Write-Host "Memory Usage: $($data.memory.usage_percent)%"
```

### CSV Processing

**Excel:** Open directly in Excel for analysis
**PowerShell:** `Import-Csv systrix_export_20251207_143022.csv`
**Python:** `pandas.read_csv('systrix_export_20251207_143022.csv')`

## 🐛 Troubleshooting

### Export Fails

**Error:** "No write permission"
**Solution:** Run as Administrator or change directory

**Error:** "Disk full"
**Solution:** Free up disk space or export to different location

**Error:** "File already open"
**Solution:** Close file in Excel/editor and try again

### Large Files

**Issue:** Export file too large
**Solution:** Reduce process limit:
```bash
systrix export --process-limit 50
```

**Issue:** Export takes too long
**Solution:** Export without processes:
```bash
systrix export --processes false
```

## ✅ Testing Checklist

After build completes:

### CLI Testing
- [ ] `systrix export` (default JSON)
- [ ] `systrix export --format csv`
- [ ] `systrix export --output test.json`
- [ ] `systrix export --processes false`
- [ ] `systrix export --process-limit 10`

### TUI Testing
- [ ] Launch TUI: `systrix tui`
- [ ] Press `e` (JSON export)
- [ ] Check success modal
- [ ] Press `Ctrl+C` (CSV export)
- [ ] Check file created

### File Verification
- [ ] Open JSON file - valid JSON format
- [ ] Open CSV file - readable in Excel
- [ ] Check all data sections present
- [ ] Verify process count matches limit

## 🎉 Benefits

1. **Data Portability** - Export for analysis in other tools
2. **Reporting** - Generate system reports for documentation
3. **Monitoring** - Integrate with monitoring systems
4. **Troubleshooting** - Capture system state for analysis
5. **Compliance** - Generate audit reports
6. **Automation** - Script system monitoring

---

**Export feature makes Systrix data accessible everywhere! 📊📁**