// src/lib.rs
//! Systrix library - System monitoring functionality
//! 
//! This library provides system monitoring capabilities that can be used
//! by both the CLI and TUI interfaces, as well as by external applications.

pub mod monitor;
pub mod utils;
pub mod plugins;

#[cfg(feature = "remote")]
pub mod remote_agent;
