// src/monitor/disk.rs
//! Disk monitoring functionality.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::{Disks, System};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSnapshot {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub fs_type: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub is_removable: bool,
}

pub async fn get_disk_snapshot(_system: &Arc<Mutex<System>>) -> Result<DiskSnapshot> {
    let disks = Disks::new_with_refreshed_list();
    
    let mut total = 0u64;
    let mut available = 0u64;
    
    for disk in disks.list() {
        total += disk.total_space();
        available += disk.available_space();
    }
    
    let used = total.saturating_sub(available);
    let usage_percent = if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    
    Ok(DiskSnapshot {
        total,
        used,
        available,
        usage_percent,
    })
}

pub async fn get_disk_list(_system: &Arc<Mutex<System>>) -> Result<Vec<DiskInfo>> {
    let disks = Disks::new_with_refreshed_list();
    
    let mut disk_list = Vec::new();
    
    for disk in disks.list() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        
        disk_list.push(DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            fs_type: String::from_utf8_lossy(disk.file_system()).to_string(),
            total,
            used,
            available,
            usage_percent,
            is_removable: disk.is_removable(),
        });
    }
    
    Ok(disk_list)
}
