# Debug Battery Parsing Issue

## Problem
Battery showing "Unknown" status with 0% after fix attempt.

## Root Cause Analysis

### WMIC Output Format
```cmd
wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime /format:csv
```

**Actual Output:**
```
Node,BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime
NOPALLL,1,80,129
```

**Possible Issues:**
1. Empty lines or whitespace
2. Different line endings (CRLF vs LF)
3. Extra blank lines in output
4. Header detection failing

## Fix Applied (v3)

### Strategy
1. Filter out ALL empty lines
2. Trim all whitespace
3. Skip any line containing "Node" or "BatteryStatus" (headers)
4. Loop through remaining lines to find valid data
5. Validate battery_status is a number before using

### Code
```rust
// Filter empty lines
let lines: Vec<&str> = output_str
    .lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .collect();

// Loop through lines, skip headers
for line in lines.iter().skip(1) {
    if line.contains("Node") || line.contains("BatteryStatus") {
        continue;
    }
    
    let parts: Vec<&str> = line.split(',').map(|p| p.trim()).collect();
    
    if parts.len() >= 4 {
        if let Ok(battery_status) = parts[1].parse::<u32>() {
            let percentage = parts[2].parse::<f32>().unwrap_or(0.0);
            // ... rest of parsing
        }
    }
}
```

## Testing Steps

### 1. Manual WMIC Test
```cmd
wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime /format:csv
```

Expected output:
```
Node,BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime
NOPALLL,1,80,129
```

### 2. Check Line Count
```powershell
$output = wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime /format:csv
$lines = $output -split "`n"
Write-Host "Total lines: $($lines.Count)"
for ($i = 0; $i -lt $lines.Count; $i++) {
    Write-Host "Line $i : [$($lines[$i])]"
}
```

### 3. Parse Data
```powershell
$dataLine = $lines[1].Trim()
$parts = $dataLine -split ","
Write-Host "Parts count: $($parts.Count)"
Write-Host "Node: [$($parts[0])]"
Write-Host "Status: [$($parts[1])]"
Write-Host "Percentage: [$($parts[2])]"
Write-Host "Time: [$($parts[3])]"
```

### 4. Test Systrix
```bash
# After build completes
.\target\release\systrix.exe tui

# Press 1 for Overview
# Check battery display
```

## Expected Results

**Before Fix:**
- Status: Unknown
- Percentage: 0%
- ❌ Not working

**After Fix:**
- Status: Discharging (or Charging)
- Percentage: 80% (actual value)
- Time: 2h 9m
- ✅ Working correctly

## Alternative Approach

If current fix doesn't work, we can try simpler WMIC format:

```rust
// Use LIST format instead of CSV
let output = Command::new("wmic")
    .args(&["path", "Win32_Battery", "get", "BatteryStatus,EstimatedChargeRemaining"])
    .output()?;

// Parse table format:
// BatteryStatus  EstimatedChargeRemaining
// 1              80
```

## Debug Output

To add debug logging (for development):

```rust
// Add at start of function
eprintln!("=== Battery Debug ===");
eprintln!("Output: {:?}", output_str);
eprintln!("Lines count: {}", lines.len());
for (i, line) in lines.iter().enumerate() {
    eprintln!("Line {}: [{}]", i, line);
}
```

## Status
- ✅ Fix v3 applied
- ⏳ Build in progress
- ⏳ Testing pending

---

**Next**: After build completes, test TUI to verify battery percentage is correct.
