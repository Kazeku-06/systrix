// src/monitor/process.rs
//! Process monitoring and management.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, Signal, System, UpdateKind};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_read: u64,
    pub disk_write: u64,
    pub threads: usize,
    pub status: String,
    pub start_time: u64,
    pub exe_path: String,
}

pub async fn get_process_list(
    system: &Arc<Mutex<System>>,
    filter: Option<&str>,
    _sort: &str,
    _limit: usize,
) -> Result<Vec<ProcessInfo>> {
    let mut sys = system.lock().await;
    
    // Refresh process list
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
    
    let mut processes = Vec::new();
    
    for (pid, process) in sys.processes() {
        let name = process.name().to_string_lossy().to_string();
        
        // Apply filter if provided
        if let Some(filter_str) = filter {
            if !name.to_lowercase().contains(&filter_str.to_lowercase()) {
                continue;
            }
        }
        
        let user = process.user_id()
            .and_then(|uid| sys.get_user_by_id(uid))
            .map(|u| u.name().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        let disk_usage = process.disk_usage();
        
        processes.push(ProcessInfo {
            pid: pid.as_u32(),
            name,
            user,
            cpu_usage: process.cpu_usage(),
            memory_usage: (process.memory() as f32 / sys.total_memory() as f32) * 100.0,
            disk_read: disk_usage.total_read_bytes,
            disk_write: disk_usage.total_written_bytes,
            threads: process.tasks().map(|t| t.len()).unwrap_or(0),
            status: format!("{:?}", process.status()),
            start_time: process.start_time(),
            exe_path: process.exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "N/A".to_string()),
        });
    }
    
    Ok(processes)
}

pub async fn kill_process(
    system: &Arc<Mutex<System>>,
    pid: u32,
    signal: &str,
) -> Result<()> {
    let sys = system.lock().await;
    
    let pid = Pid::from_u32(pid);
    
    let process = sys.process(pid)
        .ok_or_else(|| anyhow::anyhow!("Process {} not found", pid.as_u32()))?;
    
    let sig = match signal.to_uppercase().as_str() {
        "SIGTERM" | "TERM" => Signal::Term,
        "SIGKILL" | "KILL" => Signal::Kill,
        "SIGINT" | "INT" => Signal::Interrupt,
        _ => Signal::Term,
    };
    
    if process.kill_with(sig).is_some() {
        Ok(())
    } else {
        anyhow::bail!("Failed to send signal to process {}", pid.as_u32())
    }
}

pub async fn suspend_process(
    system: &Arc<Mutex<System>>,
    pid: u32,
) -> Result<()> {
    let sys = system.lock().await;
    
    let pid = Pid::from_u32(pid);
    
    let process = sys.process(pid)
        .ok_or_else(|| anyhow::anyhow!("Process {} not found", pid.as_u32()))?;
    
    #[cfg(unix)]
    {
        if process.kill_with(Signal::Stop).is_some() {
            Ok(())
        } else {
            anyhow::bail!("Failed to suspend process {}", pid.as_u32())
        }
    }
    
    #[cfg(not(unix))]
    {
        anyhow::bail!("Suspend not supported on this platform")
    }
}

pub async fn resume_process(
    system: &Arc<Mutex<System>>,
    pid: u32,
) -> Result<()> {
    let sys = system.lock().await;
    
    let pid = Pid::from_u32(pid);
    
    let process = sys.process(pid)
        .ok_or_else(|| anyhow::anyhow!("Process {} not found", pid.as_u32()))?;
    
    #[cfg(unix)]
    {
        if process.kill_with(Signal::Continue).is_some() {
            Ok(())
        } else {
            anyhow::bail!("Failed to resume process {}", pid.as_u32())
        }
    }
    
    #[cfg(not(unix))]
    {
        anyhow::bail!("Resume not supported on this platform")
    }
}
