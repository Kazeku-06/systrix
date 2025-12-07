// src/monitor/cpu.rs
//! CPU monitoring functionality.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, System};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSnapshot {
    pub model: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub global_usage: f32,
    pub frequency: f64,
    pub per_core_usage: Vec<f32>,
    pub load_average: (f64, f64, f64),
    pub uptime: u64,
    pub os_name: String,
    pub hostname: String,
}

pub async fn get_cpu_snapshot(system: &Arc<Mutex<System>>) -> Result<CpuSnapshot> {
    let mut sys = system.lock().await;
    
    // Refresh CPU info
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    sys.refresh_memory();
    
    // Wait a bit for CPU usage calculation
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    
    let cpus = sys.cpus();
    let global_usage = sys.global_cpu_info().cpu_usage();
    let per_core_usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
    
    let model = cpus.first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    
    let physical_cores = sys.physical_core_count().unwrap_or(1);
    let logical_cores = cpus.len();
    
    let frequency = cpus.first()
        .map(|cpu| cpu.frequency() as f64)
        .unwrap_or(0.0);
    
    let load_average = System::load_average();
    let uptime = System::uptime();
    
    let os_name = format!(
        "{} {}",
        System::name().unwrap_or_else(|| "Unknown".to_string()),
        System::os_version().unwrap_or_else(|| "".to_string())
    );
    
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    
    Ok(CpuSnapshot {
        model,
        physical_cores,
        logical_cores,
        global_usage,
        frequency,
        per_core_usage,
        load_average: (load_average.one, load_average.five, load_average.fifteen),
        uptime,
        os_name,
        hostname,
    })
}
