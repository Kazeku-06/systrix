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
    Html,
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
            ExportFormat::Html => "html",
        }
    );
    let filename = path.unwrap_or(&default_filename);

    match format {
        ExportFormat::Csv => export_csv(cpu, memory, disk, network, battery, processes, disk_list, filename),
        ExportFormat::Json => export_json(cpu, memory, disk, network, battery, processes, disk_list, filename),
        ExportFormat::Html => export_html(cpu, memory, disk, network, battery, processes, disk_list, filename),
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

fn export_html(
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
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    // Start HTML
    writeln!(file, "<!DOCTYPE html>")?;
    writeln!(file, "<html lang=\"en\">")?;
    writeln!(file, "<head>")?;
    writeln!(file, "    <meta charset=\"UTF-8\">")?;
    writeln!(file, "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")?;
    writeln!(file, "    <title>Systrix System Report - {}</title>", timestamp)?;
    writeln!(file, "    <style>")?;
    
    // CSS
    writeln!(file, "{}", include_str!("../assets/export_style.css"))?;
    
    writeln!(file, "    </style>")?;
    writeln!(file, "</head>")?;
    writeln!(file, "<body>")?;
    
    // Header
    writeln!(file, "    <div class=\"container\">")?;
    writeln!(file, "        <header>")?;
    writeln!(file, "            <h1>🖥️ Systrix System Report</h1>")?;
    writeln!(file, "            <p class=\"timestamp\">Generated: {}</p>", timestamp)?;
    if let Some(cpu_data) = cpu {
        writeln!(file, "            <p class=\"device-name\">Device: {}</p>", cpu_data.hostname)?;
    }
    writeln!(file, "        </header>")?;
    
    // Summary Cards
    writeln!(file, "        <div class=\"summary-grid\">")?;
    
    // CPU Card
    if let Some(cpu_data) = cpu {
        let cpu_color = if cpu_data.global_usage > 80.0 { "red" } else if cpu_data.global_usage > 60.0 { "yellow" } else { "green" };
        writeln!(file, "            <div class=\"card\">")?;
        writeln!(file, "                <h2>💻 CPU</h2>")?;
        writeln!(file, "                <div class=\"metric\">")?;
        writeln!(file, "                    <span class=\"label\">Usage</span>")?;
        writeln!(file, "                    <span class=\"value {}\">{:.1}%</span>", cpu_color, cpu_data.global_usage)?;
        writeln!(file, "                </div>")?;
        writeln!(file, "                <div class=\"progress-bar\">")?;
        writeln!(file, "                    <div class=\"progress-fill {}\" style=\"width: {:.1}%\"></div>", cpu_color, cpu_data.global_usage)?;
        writeln!(file, "                </div>")?;
        writeln!(file, "                <p class=\"detail\">{}</p>", cpu_data.model)?;
        writeln!(file, "                <p class=\"detail\">{} cores ({} logical)</p>", cpu_data.physical_cores, cpu_data.logical_cores)?;
        writeln!(file, "            </div>")?;
    }
    
    // Memory Card
    if let Some(mem_data) = memory {
        let mem_color = if mem_data.usage_percent > 80.0 { "red" } else if mem_data.usage_percent > 60.0 { "yellow" } else { "green" };
        writeln!(file, "            <div class=\"card\">")?;
        writeln!(file, "                <h2>💾 Memory</h2>")?;
        writeln!(file, "                <div class=\"metric\">")?;
        writeln!(file, "                    <span class=\"label\">Usage</span>")?;
        writeln!(file, "                    <span class=\"value {}\">{:.1}%</span>", mem_color, mem_data.usage_percent)?;
        writeln!(file, "                </div>")?;
        writeln!(file, "                <div class=\"progress-bar\">")?;
        writeln!(file, "                    <div class=\"progress-fill {}\" style=\"width: {:.1}%\"></div>", mem_color, mem_data.usage_percent)?;
        writeln!(file, "                </div>")?;
        writeln!(file, "                <p class=\"detail\">{} / {}</p>", 
            crate::utils::format_bytes(mem_data.used),
            crate::utils::format_bytes(mem_data.total))?;
        writeln!(file, "            </div>")?;
    }
    
    // Disk Card
    if let Some(disk_data) = disk {
        let disk_color = if disk_data.usage_percent > 80.0 { "red" } else if disk_data.usage_percent > 60.0 { "yellow" } else { "green" };
        writeln!(file, "            <div class=\"card\">")?;
        writeln!(file, "                <h2>💿 Disk</h2>")?;
        writeln!(file, "                <div class=\"metric\">")?;
        writeln!(file, "                    <span class=\"label\">Usage</span>")?;
        writeln!(file, "                    <span class=\"value {}\">{:.1}%</span>", disk_color, disk_data.usage_percent)?;
        writeln!(file, "                </div>")?;
        writeln!(file, "                <div class=\"progress-bar\">")?;
        writeln!(file, "                    <div class=\"progress-fill {}\" style=\"width: {:.1}%\"></div>", disk_color, disk_data.usage_percent)?;
        writeln!(file, "                </div>")?;
        writeln!(file, "                <p class=\"detail\">{} / {}</p>", 
            crate::utils::format_bytes(disk_data.used),
            crate::utils::format_bytes(disk_data.total))?;
        writeln!(file, "            </div>")?;
    }
    
    // Battery Card
    if let Some(bat_data) = battery {
        if bat_data.is_present {
            let bat_color = if bat_data.percentage < 20.0 { "red" } else if bat_data.percentage < 50.0 { "yellow" } else { "green" };
            let bat_icon = if bat_data.is_charging { "⚡" } else { "🔋" };
            writeln!(file, "            <div class=\"card\">")?;
            writeln!(file, "                <h2>{} Battery</h2>", bat_icon)?;
            writeln!(file, "                <div class=\"metric\">")?;
            writeln!(file, "                    <span class=\"label\">{}</span>", bat_data.status)?;
            writeln!(file, "                    <span class=\"value {}\">{:.0}%</span>", bat_color, bat_data.percentage)?;
            writeln!(file, "                </div>")?;
            writeln!(file, "                <div class=\"progress-bar\">")?;
            writeln!(file, "                    <div class=\"progress-fill {}\" style=\"width: {:.0}%\"></div>", bat_color, bat_data.percentage)?;
            writeln!(file, "                </div>")?;
            if let Some(time) = bat_data.time_remaining {
                writeln!(file, "                <p class=\"detail\">Time: {}</p>", crate::monitor::battery::format_time_remaining(time))?;
            }
            writeln!(file, "                <p class=\"detail\">Health: {:.0}%</p>", bat_data.health)?;
            writeln!(file, "            </div>")?;
        }
    }
    
    writeln!(file, "        </div>")?;
    
    // Detailed Information
    writeln!(file, "        <div class=\"details-section\">")?;
    
    // System Info
    if let Some(cpu_data) = cpu {
        writeln!(file, "            <div class=\"card\">")?;
        writeln!(file, "                <h2>ℹ️ System Information</h2>")?;
        writeln!(file, "                <table>")?;
        writeln!(file, "                    <tr><td>Device</td><td>{}</td></tr>", cpu_data.hostname)?;
        writeln!(file, "                    <tr><td>OS</td><td>{}</td></tr>", cpu_data.os_name)?;
        writeln!(file, "                    <tr><td>Uptime</td><td>{}</td></tr>", crate::utils::format_duration(cpu_data.uptime))?;
        writeln!(file, "                    <tr><td>CPU Frequency</td><td>{:.0} MHz</td></tr>", cpu_data.frequency)?;
        writeln!(file, "                </table>")?;
        writeln!(file, "            </div>")?;
    }
    
    // Network Info
    if let Some(net_data) = network {
        writeln!(file, "            <div class=\"card\">")?;
        writeln!(file, "                <h2>🌐 Network</h2>")?;
        writeln!(file, "                <table>")?;
        writeln!(file, "                    <tr><td>Total RX</td><td>{}</td></tr>", crate::utils::format_bytes(net_data.total_rx))?;
        writeln!(file, "                    <tr><td>Total TX</td><td>{}</td></tr>", crate::utils::format_bytes(net_data.total_tx))?;
        writeln!(file, "                </table>")?;
        if !net_data.interfaces.is_empty() {
            writeln!(file, "                <h3>Interfaces</h3>")?;
            writeln!(file, "                <table class=\"interfaces-table\">")?;
            writeln!(file, "                    <tr><th>Name</th><th>RX</th><th>TX</th></tr>")?;
            for iface in &net_data.interfaces {
                writeln!(file, "                    <tr><td>{}</td><td>{}</td><td>{}</td></tr>", 
                    iface.name,
                    crate::utils::format_bytes(iface.received),
                    crate::utils::format_bytes(iface.transmitted))?;
            }
            writeln!(file, "                </table>")?;
        }
        writeln!(file, "            </div>")?;
    }
    
    // Disk Partitions
    if !disk_list.is_empty() {
        writeln!(file, "            <div class=\"card\">")?;
        writeln!(file, "                <h2>💿 Disk Partitions</h2>")?;
        writeln!(file, "                <table class=\"disk-table\">")?;
        writeln!(file, "                    <tr><th>Name</th><th>Mount</th><th>Type</th><th>Size</th><th>Used</th><th>Usage</th></tr>")?;
        for disk in disk_list {
            let disk_color = if disk.usage_percent > 80.0 { "red" } else if disk.usage_percent > 60.0 { "yellow" } else { "green" };
            writeln!(file, "                    <tr>")?;
            writeln!(file, "                        <td>{}</td>", disk.name)?;
            writeln!(file, "                        <td>{}</td>", disk.mount_point)?;
            writeln!(file, "                        <td>{}</td>", disk.fs_type)?;
            writeln!(file, "                        <td>{}</td>", crate::utils::format_bytes(disk.total))?;
            writeln!(file, "                        <td>{}</td>", crate::utils::format_bytes(disk.used))?;
            writeln!(file, "                        <td class=\"{}\"><strong>{:.1}%</strong></td>", disk_color, disk.usage_percent)?;
            writeln!(file, "                    </tr>")?;
        }
        writeln!(file, "                </table>")?;
        writeln!(file, "            </div>")?;
    }
    
    writeln!(file, "        </div>")?;
    
    // Processes Table
    if !processes.is_empty() {
        writeln!(file, "        <div class=\"card processes-card\">")?;
        writeln!(file, "            <h2>📋 Top Processes ({} total)</h2>", processes.len())?;
        writeln!(file, "            <div class=\"table-controls\">")?;
        writeln!(file, "                <input type=\"text\" id=\"processSearch\" placeholder=\"Search processes...\" onkeyup=\"filterProcesses()\">")?;
        writeln!(file, "            </div>")?;
        writeln!(file, "            <div class=\"table-wrapper\">")?;
        writeln!(file, "                <table id=\"processTable\" class=\"process-table\">")?;
        writeln!(file, "                    <thead>")?;
        writeln!(file, "                        <tr>")?;
        writeln!(file, "                            <th onclick=\"sortTable(0)\">PID ▼</th>")?;
        writeln!(file, "                            <th onclick=\"sortTable(1)\">Name ▼</th>")?;
        writeln!(file, "                            <th onclick=\"sortTable(2)\">User ▼</th>")?;
        writeln!(file, "                            <th onclick=\"sortTable(3)\">CPU % ▼</th>")?;
        writeln!(file, "                            <th onclick=\"sortTable(4)\">Memory % ▼</th>")?;
        writeln!(file, "                            <th onclick=\"sortTable(5)\">Threads ▼</th>")?;
        writeln!(file, "                            <th onclick=\"sortTable(6)\">Status ▼</th>")?;
        writeln!(file, "                        </tr>")?;
        writeln!(file, "                    </thead>")?;
        writeln!(file, "                    <tbody>")?;
        
        for proc in processes.iter().take(100) {
            let cpu_class = if proc.cpu_usage > 50.0 { "high" } else if proc.cpu_usage > 20.0 { "medium" } else { "" };
            let mem_class = if proc.memory_usage > 50.0 { "high" } else if proc.memory_usage > 20.0 { "medium" } else { "" };
            
            writeln!(file, "                        <tr>")?;
            writeln!(file, "                            <td>{}</td>", proc.pid)?;
            writeln!(file, "                            <td class=\"process-name\">{}</td>", proc.name)?;
            writeln!(file, "                            <td>{}</td>", proc.user)?;
            writeln!(file, "                            <td class=\"{}\"><strong>{:.1}%</strong></td>", cpu_class, proc.cpu_usage)?;
            writeln!(file, "                            <td class=\"{}\"><strong>{:.1}%</strong></td>", mem_class, proc.memory_usage)?;
            writeln!(file, "                            <td>{}</td>", proc.threads)?;
            writeln!(file, "                            <td>{}</td>", proc.status)?;
            writeln!(file, "                        </tr>")?;
        }
        
        writeln!(file, "                    </tbody>")?;
        writeln!(file, "                </table>")?;
        writeln!(file, "            </div>")?;
        writeln!(file, "        </div>")?;
    }
    
    // Footer
    writeln!(file, "        <footer>")?;
    writeln!(file, "            <p>Generated by <strong>Systrix v{}</strong> - System Monitor</p>", env!("CARGO_PKG_VERSION"))?;
    writeln!(file, "            <p>Report generated at {}</p>", timestamp)?;
    writeln!(file, "        </footer>")?;
    writeln!(file, "    </div>")?;
    
    // JavaScript
    writeln!(file, "    <script>")?;
    writeln!(file, "{}", include_str!("../assets/export_script.js"))?;
    writeln!(file, "    </script>")?;
    
    writeln!(file, "</body>")?;
    writeln!(file, "</html>")?;
    
    Ok(filename.to_string())
}

/// Get default export directory
pub fn get_export_dir() -> String {
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string())
}
