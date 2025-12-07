// src/monitor/mod.rs
//! System monitoring backends and data structures.

pub mod battery;
pub mod cpu;
pub mod memory;
pub mod disk;
pub mod network;
pub mod process;

use anyhow::Result;
use async_trait::async_trait;

pub use battery::BatteryInfo;
pub use cpu::CpuSnapshot;
pub use memory::MemorySnapshot;
pub use disk::{DiskSnapshot, DiskInfo};
pub use network::NetworkSnapshot;
pub use process::ProcessInfo;

/// Trait for system monitoring backends
#[async_trait]
pub trait MonitorBackend: Send + Sync {
    async fn cpu_snapshot(&self) -> Result<CpuSnapshot>;
    async fn memory_snapshot(&self) -> Result<MemorySnapshot>;
    async fn disk_snapshot(&self) -> Result<DiskSnapshot>;
    async fn disk_list(&self) -> Result<Vec<DiskInfo>>;
    async fn network_snapshot(&self) -> Result<NetworkSnapshot>;
    async fn process_list(&self, filter: Option<&str>, sort: &str, limit: usize) -> Result<Vec<ProcessInfo>>;
}

/// Process management operations
#[async_trait]
pub trait ProcessManager: Send + Sync {
    async fn kill_process(&self, pid: u32, signal: &str) -> Result<()>;
    async fn suspend_process(&self, pid: u32) -> Result<()>;
    async fn resume_process(&self, pid: u32) -> Result<()>;
}

/// Real implementation using sysinfo crate
pub struct SysinfoBackend {
    system: std::sync::Arc<tokio::sync::Mutex<sysinfo::System>>,
}

impl SysinfoBackend {
    pub fn new() -> Self {
        Self {
            system: std::sync::Arc::new(tokio::sync::Mutex::new(sysinfo::System::new_all())),
        }
    }
}

impl Default for SysinfoBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MonitorBackend for SysinfoBackend {
    async fn cpu_snapshot(&self) -> Result<CpuSnapshot> {
        cpu::get_cpu_snapshot(&self.system).await
    }

    async fn memory_snapshot(&self) -> Result<MemorySnapshot> {
        memory::get_memory_snapshot(&self.system).await
    }

    async fn disk_snapshot(&self) -> Result<DiskSnapshot> {
        disk::get_disk_snapshot(&self.system).await
    }

    async fn disk_list(&self) -> Result<Vec<DiskInfo>> {
        disk::get_disk_list(&self.system).await
    }

    async fn network_snapshot(&self) -> Result<NetworkSnapshot> {
        network::get_network_snapshot(&self.system).await
    }

    async fn process_list(&self, filter: Option<&str>, sort: &str, limit: usize) -> Result<Vec<ProcessInfo>> {
        process::get_process_list(&self.system, filter, sort, limit).await
    }
}

#[async_trait]
impl ProcessManager for SysinfoBackend {
    async fn kill_process(&self, pid: u32, signal: &str) -> Result<()> {
        process::kill_process(&self.system, pid, signal).await
    }

    async fn suspend_process(&self, pid: u32) -> Result<()> {
        process::suspend_process(&self.system, pid).await
    }

    async fn resume_process(&self, pid: u32) -> Result<()> {
        process::resume_process(&self.system, pid).await
    }
}
