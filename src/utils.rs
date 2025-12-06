// src/utils.rs
//! Utility functions for formatting and conversions.

use std::time::Duration;

/// Format bytes into human-readable string (KB, MB, GB, etc.)
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let i = (bytes_f.log10() / 1024_f64.log10()).floor() as usize;
    let i = i.min(UNITS.len() - 1);
    
    let value = bytes_f / 1024_f64.powi(i as i32);
    
    if i == 0 {
        format!("{} {}", bytes, UNITS[i])
    } else {
        format!("{:.1} {}", value, UNITS[i])
    }
}

/// Format duration into human-readable string
pub fn format_duration(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    let days = duration.as_secs() / 86400;
    let hours = (duration.as_secs() % 86400) / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let secs = duration.as_secs() % 60;
    
    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Create a simple bar chart string
#[allow(dead_code)]
pub fn create_bar(percentage: f32, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f32) as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    
    format!("{}{}", "▇".repeat(filled), "░".repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
        assert_eq!(format_duration(86400), "1d 0h 0m");
    }

    #[test]
    fn test_create_bar() {
        assert_eq!(create_bar(0.0, 10), "░░░░░░░░░░");
        assert_eq!(create_bar(50.0, 10), "▇▇▇▇▇░░░░░");
        assert_eq!(create_bar(100.0, 10), "▇▇▇▇▇▇▇▇▇▇");
    }
}
