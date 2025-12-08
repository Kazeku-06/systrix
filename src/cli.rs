// src/cli.rs
//! CLI argument parsing and command execution using clap v4.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::monitor::{MonitorBackend, SysinfoBackend};
use crate::utils;

#[derive(Parser)]
#[command(name = "systrix")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display system information summary
    Info,
    
    /// List processes
    Ps {
        /// Sort by: cpu, mem, io
        #[arg(long, default_value = "cpu")]
        sort: String,
        
        /// Filter processes by name
        #[arg(long)]
        filter: Option<String>,
        
        /// Limit number of results
        #[arg(long, default_value = "20")]
        limit: usize,
    },
    
    /// Kill a process
    Kill {
        /// Process ID to kill
        pid: u32,
        
        /// Signal to send: SIGTERM, SIGKILL
        #[arg(long, default_value = "SIGTERM")]
        signal: String,
        
        /// Force kill without confirmation
        #[arg(long)]
        force: bool,
    },
    
    /// Display network interfaces
    Net,
    
    /// Display disk partitions and usage
    Disk,
    
    /// Launch interactive TUI
    #[cfg(feature = "tui")]
    Tui {
        /// Refresh interval in milliseconds
        #[arg(long, default_value = "500")]
        refresh_interval: u64,
    },
    
    /// Export system report
    Report {
        /// Output file path
        #[arg(long, default_value = "report.json")]
        output: PathBuf,
    },
    
    /// Export system data
    Export {
        /// Export format: csv, json
        #[arg(long, default_value = "json")]
        format: String,
        
        /// Output file path (optional, auto-generated if not provided)
        #[arg(long)]
        output: Option<PathBuf>,
        
        /// Include process list
        #[arg(long, default_value = "true")]
        processes: bool,
        
        /// Maximum number of processes to include
        #[arg(long, default_value = "100")]
        process_limit: usize,
    },
    
    /// Display version information
    Version,
}

/// Execute the CLI command
pub async fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Info) => cmd_info().await,
        Some(Commands::Ps { sort, filter, limit }) => cmd_ps(sort, filter, limit).await,
        Some(Commands::Kill { pid, signal, force }) => cmd_kill(pid, signal, force).await,
        Some(Commands::Net) => cmd_net().await,
        Some(Commands::Disk) => cmd_disk().await,
        #[cfg(feature = "tui")]
        Some(Commands::Tui { refresh_interval }) => cmd_tui(refresh_interval).await,
        Some(Commands::Report { output }) => cmd_report(output).await,
        Some(Commands::Export { format, output, processes, process_limit }) => {
            cmd_export(format, output, processes, process_limit).await
        },
        Some(Commands::Version) => cmd_version(),
        None => {
            // Default: launch TUI if available, otherwise show help
            #[cfg(feature = "tui")]
            {
                cmd_tui(500).await
            }
            #[cfg(not(feature = "tui"))]
            {
                println!("Systrix v{}", env!("CARGO_PKG_VERSION"));
                println!("Use --help to see available commands");
                Ok(())
            }
        }
    }
}

async fn cmd_info() -> Result<()> {
    let backend = SysinfoBackend::new();
    let cpu = backend.cpu_snapshot().await?;
    let memory = backend.memory_snapshot().await?;
    let disk = backend.disk_snapshot().await?;
    
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                    SYSTRIX - System Info                 ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
    println!("System:");
    println!("  Device: {}", cpu.hostname);
    println!("  OS: {}", cpu.os_name);
    println!("  Uptime: {}", utils::format_duration(cpu.uptime));
    println!();
    println!("CPU:");
    println!("  Model: {}", cpu.model);
    println!("  Cores: {} physical, {} logical", cpu.physical_cores, cpu.logical_cores);
    println!("  Usage: {:.1}%", cpu.global_usage);
    println!("  Frequency: {:.0} MHz", cpu.frequency);
    println!();
    println!("Memory:");
    println!("  Total: {}", utils::format_bytes(memory.total));
    println!("  Used: {} ({:.1}%)", utils::format_bytes(memory.used), memory.usage_percent);
    println!("  Available: {}", utils::format_bytes(memory.available));
    println!();
    println!("Disk:");
    println!("  Total: {}", utils::format_bytes(disk.total));
    println!("  Used: {} ({:.1}%)", utils::format_bytes(disk.used), disk.usage_percent);
    println!("  Available: {}", utils::format_bytes(disk.available));
    
    Ok(())
}

async fn cmd_ps(sort: String, filter: Option<String>, limit: usize) -> Result<()> {
    let backend = SysinfoBackend::new();
    let mut processes = backend.process_list(filter.as_deref(), &sort, limit).await?;
    
    // Sort processes
    match sort.as_str() {
        "mem" => processes.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap()),
        "io" => processes.sort_by(|a, b| (b.disk_read + b.disk_write).partial_cmp(&(a.disk_read + a.disk_write)).unwrap()),
        _ => processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()),
    }
    
    println!("{:<8} {:<10} {:<20} {:>6} {:>6} {:>8} {:>8} {:>8}", 
             "PID", "USER", "NAME", "CPU%", "MEM%", "IO_R", "IO_W", "THREADS");
    println!("{}", "─".repeat(80));
    
    for proc in processes.iter().take(limit) {
        println!("{:<8} {:<10} {:<20} {:>5.1}% {:>5.1}% {:>8} {:>8} {:>8}",
                 proc.pid,
                 proc.user.chars().take(10).collect::<String>(),
                 proc.name.chars().take(20).collect::<String>(),
                 proc.cpu_usage,
                 proc.memory_usage,
                 utils::format_bytes(proc.disk_read),
                 utils::format_bytes(proc.disk_write),
                 proc.threads);
    }
    
    println!();
    println!("Showing {} of {} processes", processes.len().min(limit), processes.len());
    
    Ok(())
}

async fn cmd_kill(pid: u32, signal: String, force: bool) -> Result<()> {
    use crate::monitor::ProcessManager;
    
    // Safety check: prevent killing critical processes
    if pid <= 1 && !force {
        anyhow::bail!("Cannot kill system process (PID {}). Use --force to override (not recommended)", pid);
    }
    
    let backend = SysinfoBackend::new();
    
    // Confirmation
    if !force {
        println!("About to kill process {} with signal {}", pid, signal);
        println!("Continue? (y/N): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }
    
    match backend.kill_process(pid, &signal).await {
        Ok(_) => println!("✓ Process {} killed successfully", pid),
        Err(e) => {
            if e.to_string().contains("permission") {
                eprintln!("✗ Permission denied. Try running with elevated privileges (sudo/admin)");
            } else {
                eprintln!("✗ Failed to kill process: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn cmd_net() -> Result<()> {
    let backend = SysinfoBackend::new();
    let networks = backend.network_snapshot().await?;
    
    println!("Network Interfaces:");
    println!("{:<15} {:>12} {:>12} {:>12} {:>12}", 
             "INTERFACE", "RX_BYTES", "TX_BYTES", "RX_RATE", "TX_RATE");
    println!("{}", "─".repeat(70));
    
    for net in &networks.interfaces {
        println!("{:<15} {:>12} {:>12} {:>12} {:>12}",
                 net.name,
                 utils::format_bytes(net.received),
                 utils::format_bytes(net.transmitted),
                 format!("{}/s", utils::format_bytes(net.rx_rate)),
                 format!("{}/s", utils::format_bytes(net.tx_rate)));
    }
    
    Ok(())
}

async fn cmd_disk() -> Result<()> {
    let backend = SysinfoBackend::new();
    let disks = backend.disk_list().await?;
    
    println!("Disk Partitions:");
    println!("{:<20} {:<10} {:>12} {:>12} {:>12} {:>8}", 
             "MOUNT", "TYPE", "TOTAL", "USED", "AVAILABLE", "USE%");
    println!("{}", "─".repeat(80));
    
    for disk in &disks {
        println!("{:<20} {:<10} {:>12} {:>12} {:>12} {:>7.1}%",
                 disk.mount_point,
                 disk.fs_type,
                 utils::format_bytes(disk.total),
                 utils::format_bytes(disk.used),
                 utils::format_bytes(disk.available),
                 disk.usage_percent);
    }
    
    Ok(())
}

#[cfg(feature = "tui")]
async fn cmd_tui(refresh_interval: u64) -> Result<()> {
    use crate::app::App;
    
    // Clamp refresh interval to minimum 100ms
    let refresh_interval = refresh_interval.max(100);
    
    let mut app = App::new(refresh_interval)?;
    app.run().await?;
    
    Ok(())
}

async fn cmd_report(output: PathBuf) -> Result<()> {
    // Legacy report command - use new export functionality
    cmd_export("json".to_string(), Some(output), true, 50).await
}

async fn cmd_export(format: String, output: Option<PathBuf>, include_processes: bool, process_limit: usize) -> Result<()> {
    use crate::export::{export_snapshot, ExportFormat};
    
    let backend = SysinfoBackend::new();
    let cpu = Some(backend.cpu_snapshot().await?);
    let memory = Some(backend.memory_snapshot().await?);
    let disk = Some(backend.disk_snapshot().await?);
    let network = Some(backend.network_snapshot().await?);
    let battery = crate::monitor::battery::get_battery_info().await.ok();
    let disk_list = backend.disk_list().await?;
    
    let processes = if include_processes {
        backend.process_list(None, "cpu", process_limit).await?
    } else {
        Vec::new()
    };
    
    let export_format = match format.to_lowercase().as_str() {
        "csv" => ExportFormat::Csv,
        "json" => ExportFormat::Json,
        _ => {
            println!("❌ Invalid format '{}'. Supported formats: csv, json", format);
            return Ok(());
        }
    };
    
    let output_path = output.map(|p| p.to_string_lossy().to_string());
    
    match export_snapshot(
        &cpu,
        &memory,
        &disk,
        &network,
        &battery,
        &processes,
        &disk_list,
        export_format,
        output_path.as_deref(),
    ) {
        Ok(filename) => {
            // Get full path
            let full_path = std::env::current_dir()
                .map(|p| p.join(&filename).to_string_lossy().to_string())
                .unwrap_or_else(|_| filename.clone());
            
            println!("✅ Data exported successfully!");
            println!();
            println!("📁 Filename: {}", filename);
            println!("📂 Location: {}", full_path);
            println!("📊 Format: {}", format.to_uppercase());
            println!("🔢 Processes: {}", processes.len());
            if let Some(battery) = &battery {
                if battery.is_present {
                    println!("🔋 Battery: Included");
                }
            }
            println!();
            println!("💡 Tip: Open with:");
            match export_format {
                ExportFormat::Csv => println!("   Excel, LibreOffice, or any spreadsheet app"),
                ExportFormat::Json => println!("   Text editor, browser, or JSON viewer"),
            }
        }
        Err(e) => {
            println!("❌ Export failed: {}", e);
        }
    }
    
    Ok(())
}

fn cmd_version() -> Result<()> {
    println!("systrix v{}", env!("CARGO_PKG_VERSION"));
    println!("Rust System Monitor - CLI + TUI");
    println!();
    println!("Features:");
    #[cfg(feature = "tui")]
    println!("  ✓ TUI");
    #[cfg(feature = "gpu")]
    println!("  ✓ GPU monitoring");
    #[cfg(feature = "remote")]
    println!("  ✓ Remote agent");
    #[cfg(feature = "dynamic-plugins")]
    println!("  ✓ Dynamic plugins");
    
    Ok(())
}
