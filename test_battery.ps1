# Test Battery Parsing
Write-Host "=== Testing Battery Data Parsing ===" -ForegroundColor Cyan
Write-Host ""

# Get actual battery data from WMIC
Write-Host "1. Actual Battery Data from WMIC:" -ForegroundColor Yellow
$wmicOutput = wmic path Win32_Battery get BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime /format:csv
Write-Host $wmicOutput
Write-Host ""

# Parse the data
$lines = $wmicOutput -split "`n"
if ($lines.Count -ge 2) {
    $dataLine = $lines[1].Trim()
    $parts = $dataLine -split ","
    
    if ($parts.Count -ge 4) {
        Write-Host "2. Parsed Data:" -ForegroundColor Yellow
        Write-Host "   Node: $($parts[0])"
        Write-Host "   Battery Status: $($parts[1])"
        Write-Host "   Percentage: $($parts[2])%"
        Write-Host "   Time Remaining: $($parts[3]) minutes"
        Write-Host ""
        
        # Convert time to hours and minutes
        $minutes = [int]$parts[3]
        $hours = [math]::Floor($minutes / 60)
        $mins = $minutes % 60
        Write-Host "   Time Remaining: ${hours}h ${mins}m" -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "3. Now run Systrix TUI to verify:" -ForegroundColor Yellow
Write-Host "   .\target\release\systrix.exe tui" -ForegroundColor Cyan
Write-Host "   Press 1 for Overview panel"
Write-Host "   Check if battery percentage matches the value above"
Write-Host ""
