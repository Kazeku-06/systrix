// tests/monitor_tests.rs
//! Integration tests for monitoring functionality.

use systrix::monitor::{MonitorBackend, SysinfoBackend};

#[allow(unused_imports)]
use systrix::monitor::ProcessManager;

#[tokio::test]
async fn test_cpu_snapshot() {
    let backend = SysinfoBackend::new();
    let result = backend.cpu_snapshot().await;
    
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    
    assert!(!snapshot.model.is_empty());
    assert!(snapshot.physical_cores > 0);
    assert!(snapshot.logical_cores > 0);
    assert!(snapshot.global_usage >= 0.0);
    assert!(snapshot.global_usage <= 100.0);
}

#[tokio::test]
async fn test_memory_snapshot() {
    let backend = SysinfoBackend::new();
    let result = backend.memory_snapshot().await;
    
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    
    assert!(snapshot.total > 0);
    assert!(snapshot.used <= snapshot.total);
    assert!(snapshot.available <= snapshot.total);
    assert!(snapshot.usage_percent >= 0.0);
    assert!(snapshot.usage_percent <= 100.0);
}

#[tokio::test]
async fn test_disk_snapshot() {
    let backend = SysinfoBackend::new();
    let result = backend.disk_snapshot().await;
    
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    
    assert!(snapshot.total > 0);
    assert!(snapshot.used <= snapshot.total);
    assert!(snapshot.usage_percent >= 0.0);
    assert!(snapshot.usage_percent <= 100.0);
}

#[tokio::test]
async fn test_network_snapshot() {
    let backend = SysinfoBackend::new();
    let result = backend.network_snapshot().await;
    
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    
    // Network interfaces may be empty on some systems
    assert!(snapshot.total_rx >= 0);
    assert!(snapshot.total_tx >= 0);
}

#[tokio::test]
async fn test_process_list() {
    let backend = SysinfoBackend::new();
    let result = backend.process_list(None, "cpu", 10).await;
    
    assert!(result.is_ok());
    let processes = result.unwrap();
    
    // Should have at least one process (the test itself)
    assert!(!processes.is_empty());
    
    for proc in &processes {
        assert!(proc.pid > 0);
        assert!(!proc.name.is_empty());
    }
}

#[tokio::test]
async fn test_process_list_with_filter() {
    let backend = SysinfoBackend::new();
    
    // Filter for a common process name
    let result = backend.process_list(Some("test"), "cpu", 10).await;
    
    assert!(result.is_ok());
    let processes = result.unwrap();
    
    // All returned processes should match the filter
    for proc in &processes {
        assert!(proc.name.to_lowercase().contains("test"));
    }
}

#[tokio::test]
async fn test_disk_list() {
    let backend = SysinfoBackend::new();
    let result = backend.disk_list().await;
    
    assert!(result.is_ok());
    let disks = result.unwrap();
    
    // Should have at least one disk
    assert!(!disks.is_empty());
    
    for disk in &disks {
        assert!(!disk.mount_point.is_empty());
        assert!(disk.total > 0);
    }
}
