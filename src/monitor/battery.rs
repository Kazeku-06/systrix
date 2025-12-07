// src/monitor/battery.rs
//! Battery monitoring functionality.

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub is_present: bool,
    pub percentage: f32,
    pub is_charging: bool,
    pub is_plugged: bool,
    pub time_remaining: Option<u64>, // in seconds
    pub health: f32, // 0-100%
    pub status: String,
    pub technology: String,
    pub vendor: String,
}

impl Default for BatteryInfo {
    fn default() -> Self {
        Self {
            is_present: false,
            percentage: 0.0,
            is_charging: false,
            is_plugged: false,
            time_remaining: None,
            health: 100.0,
            status: "Unknown".to_string(),
            technology: "Unknown".to_string(),
            vendor: "Unknown".to_string(),
        }
    }
}

pub async fn get_battery_info() -> Result<BatteryInfo> {
    #[cfg(target_os = "windows")]
    {
        get_battery_info_windows().await
    }
    
    #[cfg(target_os = "linux")]
    {
        get_battery_info_linux().await
    }
    
    #[cfg(target_os = "macos")]
    {
        get_battery_info_macos().await
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Ok(BatteryInfo::default())
    }
}

#[cfg(target_os = "windows")]
async fn get_battery_info_windows() -> Result<BatteryInfo> {
    use std::process::Command;
    
    // Use WMIC to get battery info
    let output = Command::new("wmic")
        .args(&["path", "Win32_Battery", "get", "BatteryStatus,EstimatedChargeRemaining,EstimatedRunTime", "/format:csv"])
        .output()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    
    // Check if battery exists
    if lines.len() < 3 {
        return Ok(BatteryInfo::default());
    }
    
    // Parse CSV output (skip header and node name)
    if let Some(data_line) = lines.get(2) {
        let parts: Vec<&str> = data_line.split(',').collect();
        
        if parts.len() >= 3 {
            let battery_status = parts[0].trim().parse::<u32>().unwrap_or(0);
            let percentage = parts[1].trim().parse::<f32>().unwrap_or(0.0);
            let time_remaining = parts[2].trim().parse::<u64>().ok();
            
            // Battery status codes:
            // 1 = Discharging, 2 = AC, 3 = Fully Charged, 4 = Low, 5 = Critical
            let is_charging = battery_status == 2;
            let is_plugged = battery_status == 2 || battery_status == 3;
            
            let status = match battery_status {
                1 => "Discharging",
                2 => "Charging",
                3 => "Fully Charged",
                4 => "Low",
                5 => "Critical",
                _ => "Unknown",
            }.to_string();
            
            return Ok(BatteryInfo {
                is_present: true,
                percentage,
                is_charging,
                is_plugged,
                time_remaining: time_remaining.filter(|&t| t != 71582788), // Filter invalid values
                health: 100.0, // Windows doesn't easily provide this
                status,
                technology: "Li-ion".to_string(), // Assume Li-ion
                vendor: "Unknown".to_string(),
            });
        }
    }
    
    Ok(BatteryInfo::default())
}

#[cfg(target_os = "linux")]
async fn get_battery_info_linux() -> Result<BatteryInfo> {
    use std::fs;
    use std::path::Path;
    
    let battery_path = Path::new("/sys/class/power_supply/BAT0");
    
    if !battery_path.exists() {
        // Try BAT1
        let battery_path = Path::new("/sys/class/power_supply/BAT1");
        if !battery_path.exists() {
            return Ok(BatteryInfo::default());
        }
    }
    
    let read_file = |name: &str| -> Option<String> {
        fs::read_to_string(battery_path.join(name)).ok()
            .map(|s| s.trim().to_string())
    };
    
    let capacity = read_file("capacity")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    
    let status = read_file("status").unwrap_or_else(|| "Unknown".to_string());
    let is_charging = status == "Charging";
    let is_plugged = status == "Charging" || status == "Full";
    
    let technology = read_file("technology").unwrap_or_else(|| "Unknown".to_string());
    let vendor = read_file("manufacturer").unwrap_or_else(|| "Unknown".to_string());
    
    // Calculate time remaining if available
    let time_remaining = if let (Some(energy_now), Some(power_now)) = 
        (read_file("energy_now"), read_file("power_now")) {
        if let (Ok(energy), Ok(power)) = (energy_now.parse::<f64>(), power_now.parse::<f64>()) {
            if power > 0.0 {
                Some(((energy / power) * 3600.0) as u64)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    
    // Calculate health
    let health = if let (Some(energy_full), Some(energy_design)) = 
        (read_file("energy_full"), read_file("energy_full_design")) {
        if let (Ok(full), Ok(design)) = (energy_full.parse::<f64>(), energy_design.parse::<f64>()) {
            if design > 0.0 {
                ((full / design) * 100.0) as f32
            } else {
                100.0
            }
        } else {
            100.0
        }
    } else {
        100.0
    };
    
    Ok(BatteryInfo {
        is_present: true,
        percentage: capacity,
        is_charging,
        is_plugged,
        time_remaining,
        health,
        status,
        technology,
        vendor,
    })
}

#[cfg(target_os = "macos")]
async fn get_battery_info_macos() -> Result<BatteryInfo> {
    use std::process::Command;
    
    let output = Command::new("pmset")
        .args(&["-g", "batt"])
        .output()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Parse output like: "Now drawing from 'Battery Power' -InternalBattery-0 (id=1234567)	95%; discharging; 5:23 remaining"
    let mut battery_info = BatteryInfo::default();
    
    for line in output_str.lines() {
        if line.contains("InternalBattery") {
            battery_info.is_present = true;
            
            // Extract percentage
            if let Some(pct_start) = line.find(char::is_numeric) {
                if let Some(pct_end) = line[pct_start..].find('%') {
                    if let Ok(pct) = line[pct_start..pct_start + pct_end].parse::<f32>() {
                        battery_info.percentage = pct;
                    }
                }
            }
            
            // Check charging status
            battery_info.is_charging = line.contains("charging") && !line.contains("discharging");
            battery_info.is_plugged = line.contains("AC Power") || battery_info.is_charging;
            
            if battery_info.is_charging {
                battery_info.status = "Charging".to_string();
            } else if line.contains("charged") {
                battery_info.status = "Fully Charged".to_string();
            } else {
                battery_info.status = "Discharging".to_string();
            }
            
            // Extract time remaining
            if let Some(time_pos) = line.find("remaining") {
                let time_str = &line[..time_pos];
                if let Some(time_start) = time_str.rfind(char::is_numeric) {
                    // Parse time like "5:23"
                    let time_part = &time_str[..=time_start];
                    if let Some(colon_pos) = time_part.rfind(':') {
                        let hours = time_part[..colon_pos].trim().parse::<u64>().unwrap_or(0);
                        let mins = time_part[colon_pos + 1..].trim().parse::<u64>().unwrap_or(0);
                        battery_info.time_remaining = Some(hours * 3600 + mins * 60);
                    }
                }
            }
        }
    }
    
    // Get health info
    let health_output = Command::new("system_profiler")
        .args(&["SPPowerDataType"])
        .output()?;
    
    let health_str = String::from_utf8_lossy(&health_output.stdout);
    for line in health_str.lines() {
        if line.contains("Condition:") {
            if line.contains("Normal") {
                battery_info.health = 100.0;
            } else if line.contains("Replace") {
                battery_info.health = 50.0;
            }
        }
    }
    
    battery_info.technology = "Li-ion".to_string();
    battery_info.vendor = "Apple".to_string();
    
    Ok(battery_info)
}

pub fn format_time_remaining(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn get_battery_icon(percentage: f32, is_charging: bool) -> &'static str {
    if is_charging {
        return "🔌";
    }
    
    match percentage as u32 {
        90..=100 => "🔋",
        60..=89 => "🔋",
        30..=59 => "🔋",
        10..=29 => "🪫",
        _ => "🪫",
    }
}

pub fn get_battery_color(percentage: f32, is_charging: bool) -> ratatui::style::Color {
    use ratatui::style::Color;
    
    if is_charging {
        return Color::Green;
    }
    
    match percentage as u32 {
        75..=100 => Color::Green,
        30..=74 => Color::Yellow,
        15..=29 => Color::LightRed,
        _ => Color::Red,
    }
}
