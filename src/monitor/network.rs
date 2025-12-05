// src/monitor/network.rs
//! Network monitoring functionality.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sysinfo::{Networks, System};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSnapshot {
    pub interfaces: Vec<NetworkInterface>,
    pub total_rx: u64,
    pub total_tx: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
    pub rx_rate: u64,
    pub tx_rate: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub errors_received: u64,
    pub errors_transmitted: u64,
}

pub async fn get_network_snapshot(_system: &Arc<Mutex<System>>) -> Result<NetworkSnapshot> {
    let networks = Networks::new_with_refreshed_list();
    
    let mut interfaces = Vec::new();
    let mut total_rx = 0u64;
    let mut total_tx = 0u64;
    
    for (name, data) in networks.list() {
        let received = data.total_received();
        let transmitted = data.total_transmitted();
        
        total_rx += received;
        total_tx += transmitted;
        
        interfaces.push(NetworkInterface {
            name: name.clone(),
            received,
            transmitted,
            rx_rate: data.received(),
            tx_rate: data.transmitted(),
            packets_received: data.total_packets_received(),
            packets_transmitted: data.total_packets_transmitted(),
            errors_received: data.total_errors_on_received(),
            errors_transmitted: data.total_errors_on_transmitted(),
        });
    }
    
    Ok(NetworkSnapshot {
        interfaces,
        total_rx,
        total_tx,
    })
}
