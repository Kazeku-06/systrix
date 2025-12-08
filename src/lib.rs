// src/lib.rs
//! Systrix library - System monitoring functionality
//! 
//! This library provides system monitoring capabilities that can be used
//! by both the CLI and TUI interfaces, as well as by external applications.

pub mod export;
pub mod monitor;
pub mod plugins;
pub mod utils;

#[cfg(feature = "remote")]
pub mod remote_agent;
