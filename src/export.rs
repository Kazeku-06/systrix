// src/export.rs
//! Data export functionality for CSV and JSON formats.

use anyhow::Result;
use chrono::Local;
use std::fs::File;
use std::io::Write;

use crate::monitor::{BatteryInfo, CpuSnapshot, DiskInfo, DiskSnapshot, MemorySnapshot, NetworkSnapshot, ProcessInfo};

/// Export format
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Csv,
    Json,
}

/// Export current system snapshot to file
pub fn export_snapshot(
    cpu: &Option<CpuSnapshot>,
    memory: &Option<MemorySnapshot>,
    disk: &Option<DiskSnapshot>,
    network: &Option<NetworkSnapshot>,
    battery: &Option<BatteryInfo>,
    processes: &[ProcessInfo],
    disk_list: &[DiskInfo],
    format: ExportFormat,
    path: Option<&str>,
) -> Result<String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let default_filename = format!(
        "systrix_export_{}.{}",
        timestamp,
        match format {
            ExportFormat::Csv => "csv",
            ExportFormat::Json => "json",
        }
    );
    let filename = path.unwrap_or(&default_filename);

    match format {
        ExportFormat::Csv => export_csv(cpu, memory, disk, network, battery, processes, disk_list, filename),
        ExportFormat::Json => export_json(cpu, memory, disk, network, battery, processes, disk_list, filename),
    }
}

fn export_csv(
    cpu: &Option<CpuSnapshot>,
    memory: &Option<MemorySnapshot>,
    disk: &Option<DiskSnapshot>,
    network: &Option<NetworkSnapshot>,
    battery: &Option<BatteryInfo>,
    processes: &[ProcessInfo],
    disk_list: &[DiskInfo],
    filename: &str,
) -> Result<String> {
    let mut file = File::create(filename)?;
    
    // Write header
    writeln!(file, "Systrix System Monitor Export")?;
    writeln!(file, "Timestamp,{}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(file)?;
    
    // System Information
    if let Some(cpu_data) = cpu {
        writeln!(file, "=== SYSTEM INFORMATION ===")?;
        writeln!(file, "Device,{}", cpu_data.hostname)?;
        writeln!(file, "OS,{}", cpu_data.os_name)?;
        writeln!(file, "Uptime (seconds),{}", cpu_data.uptime)?;
        writeln!(file)?;
    }
    
    // CPU
    if let Some(cpu_data) = cpu {
        writeln!(file, "=== CPU ===")?;
        writeln!(file, "Model,{}", cpu_data.model)?;
        writeln!(file, "Physical Cores,{}", cpu_data.physical_cores)?;
        writeln!(file, "Logical Cores,{}", cpu_data.logical_cores)?;
        writeln!(file, "Usage (%),{:.2}", cpu_data.global_usage)?;
        writeln!(file, "Frequency (MHz),{:.0}", cpu_data.frequency)?;
        writeln!(file)?;
    }
    
    // Memory
    if let Some(mem_data) = memory {
        writeln!(file, "=== MEMORY ===")?;
        writeln!(file, "Total (bytes),{}", mem_data.total)?;
        writeln!(file, "Used (bytes),{}", mem_data.used)?;
        writeln!(file, "Available (bytes),{}", mem_data.available)?;
        writeln!(file, "Usage (%),{:.2}", mem_data.usage_percent)?;
        writeln!(file)?;
    }
    
    // Disk
    if let Some(disk_data) = disk {
        writeln!(file, "=== DISK (Total) ===")?;
        writeln!(file, "Total (bytes),{}", disk_data.total)?;
        writeln!(file, "Used (bytes),{}", disk_data.used)?;
        writeln!(file, "Available (bytes),{}", disk_data.available)?;
        writeln!(file, "Usage (%),{:.2}", disk_data.usage_percent)?;
        writeln!(file)?;
    }
    
    // Disk List
    if !disk_list.is_empty() {
        writeln!(file, "=== DISK PARTITIONS ===")?;
        writeln!(file, "Name,Mount Point,Filesystem,Total (bytes),Used (bytes),Available (bytes),Usage (%)")?;
        for disk in disk_list {
            writeln!(
                file,
                "{},{},{},{},{},{},{:.2}",
                disk.name,
                disk.mount_point,
                disk.fs_type,
                disk.total,
                disk.used,
                disk.available,
                disk.usage_percent
            )?;
        }
        writeln!(file)?;
    }
    
    // Network
    if let Some(net_data) = network {
        writeln!(file, "=== NETWORK ===")?;
        writeln!(file, "Total RX (bytes),{}", net_data.total_rx)?;
        writeln!(file, "Total TX (bytes),{}", net_data.total_tx)?;
        
        if !net_data.interfaces.is_empty() {
            writeln!(file)?;
            writeln!(file, "=== NETWORK INTERFACES ===")?;
            writeln!(file, "Name,RX (bytes),TX (bytes),RX Rate (bytes/s),TX Rate (bytes/s),Packets RX,Packets TX")?;
            for iface in &net_data.interfaces {
                writeln!(
                    file,
                    "{},{},{},{},{},{},{}",
                    iface.name,
                    iface.received,
                    iface.transmitted,
                    iface.rx_rate,
                    iface.tx_rate,
                    iface.packets_received,
                    iface.packets_transmitted
                )?;
            }
        }
        writeln!(file)?;
    }
    
    // Battery
    if let Some(bat_data) = battery {
        if bat_data.is_present {
            writeln!(file, "=== BATTERY ===")?;
            writeln!(file, "Percentage,{:.0}", bat_data.percentage)?;
            writeln!(file, "Status,{}", bat_data.status)?;
            writeln!(file, "Charging,{}", bat_data.is_charging)?;
            writeln!(file, "Plugged,{}", bat_data.is_plugged)?;
            if let Some(time) = bat_data.time_remaining {
                writeln!(file, "Time Remaining (seconds),{}", time)?;
            }
            writeln!(file, "Health (%),{:.0}", bat_data.health)?;
            writeln!(file)?;
        }
    }
    
    // Processes
    if !processes.is_empty() {
        writeln!(file, "=== PROCESSES ===")?;
        writeln!(file, "PID,Name,User,CPU (%),Memory (%),Disk Read (bytes),Disk Write (bytes),Threads,Status,Executable")?;
        for proc in processes {
            writeln!(
                file,
                "{},{},{},{:.2},{:.2},{},{},{},{},{}",
                proc.pid,
                proc.name,
                proc.user,
                proc.cpu_usage,
                proc.memory_usage,
                proc.disk_read,
                proc.disk_write,
                proc.threads,
                proc.status,
                proc.exe_path
            )?;
        }
    }
    
    Ok(filename.to_string())
}

fn export_json(
    cpu: &Option<CpuSnapshot>,
    memory: &Option<MemorySnapshot>,
    disk: &Option<DiskSnapshot>,
    network: &Option<NetworkSnapshot>,
    battery: &Option<BatteryInfo>,
    processes: &[ProcessInfo],
    disk_list: &[DiskInfo],
    filename: &str,
) -> Result<String> {
    use serde_json::json;
    
    let data = json!({
        "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "system": cpu.as_ref().map(|c| json!({
            "device": c.hostname,
            "os": c.os_name,
            "uptime_seconds": c.uptime,
        })),
        "cpu": cpu.as_ref().map(|c| json!({
            "model": c.model,
            "physical_cores": c.physical_cores,
            "logical_cores": c.logical_cores,
            "usage_percent": c.global_usage,
            "frequency_mhz": c.frequency,
            "per_core_usage": c.per_core_usage,
        })),
        "memory": memory.as_ref().map(|m| json!({
            "total_bytes": m.total,
            "used_bytes": m.used,
            "available_bytes": m.available,
            "usage_percent": m.usage_percent,
        })),
        "disk": disk.as_ref().map(|d| json!({
            "total_bytes": d.total,
            "used_bytes": d.used,
            "available_bytes": d.available,
            "usage_percent": d.usage_percent,
        })),
        "disk_partitions": disk_list.iter().map(|d| json!({
            "name": d.name,
            "mount_point": d.mount_point,
            "filesystem": d.fs_type,
            "total_bytes": d.total,
            "used_bytes": d.used,
            "available_bytes": d.available,
            "usage_percent": d.usage_percent,
        })).collect::<Vec<_>>(),
        "network": network.as_ref().map(|n| json!({
            "total_rx_bytes": n.total_rx,
            "total_tx_bytes": n.total_tx,
            "interfaces": n.interfaces.iter().map(|i| json!({
                "name": i.name,
                "received_bytes": i.received,
                "transmitted_bytes": i.transmitted,
                "rx_rate_bytes_per_sec": i.rx_rate,
                "tx_rate_bytes_per_sec": i.tx_rate,
                "packets_received": i.packets_received,
                "packets_transmitted": i.packets_transmitted,
            })).collect::<Vec<_>>(),
        })),
        "battery": battery.as_ref().and_then(|b| {
            if b.is_present {
                Some(json!({
                    "percentage": b.percentage,
                    "status": b.status,
                    "is_charging": b.is_charging,
                    "is_plugged": b.is_plugged,
                    "time_remaining_seconds": b.time_remaining,
                    "health_percent": b.health,
                    "technology": b.technology,
                    "vendor": b.vendor,
                }))
            } else {
                None
            }
        }),
        "processes": processes.iter().map(|p| json!({
            "pid": p.pid,
            "name": p.name,
            "user": p.user,
            "cpu_percent": p.cpu_usage,
            "memory_percent": p.memory_usage,
            "disk_read_bytes": p.disk_read,
            "disk_write_bytes": p.disk_write,
            "threads": p.threads,
            "status": p.status,
            "executable": p.exe_path,
        })).collect::<Vec<_>>(),
    });
    
    let mut file = File::create(filename)?;
    let json_string = serde_json::to_string_pretty(&data)?;
    file.write_all(json_string.as_bytes())?;
    
    Ok(filename.to_string())
}

/// Get default export directory
pub fn get_export_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}
