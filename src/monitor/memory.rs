// src/monitor/memory.rs
//! Memory monitoring functionality.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::System;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_usage_percent: f32,
}

pub async fn get_memory_snapshot(system: &Arc<Mutex<System>>) -> Result<MemorySnapshot> {
    let mut sys = system.lock().await;
    sys.refresh_memory();
    
    let total = sys.total_memory();
    let available = sys.available_memory();
    let used = total - available;
    let usage_percent = if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();
    let swap_usage_percent = if swap_total > 0 {
        (swap_used as f32 / swap_total as f32) * 100.0
    } else {
        0.0
    };
    
    Ok(MemorySnapshot {
        total,
        used,
        available,
        usage_percent,
        swap_total,
        swap_used,
        swap_usage_percent,
    })
}
