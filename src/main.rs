// src/main.rs
//! Systrix - System Monitor Terminal App
//! 
//! Entry point for the application. Handles CLI argument parsing and
//! dispatches to appropriate subcommands or launches TUI.

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod app;
mod export;
mod monitor;
mod utils;
mod plugins;

#[cfg(feature = "tui")]
mod tui;

#[cfg(feature = "remote")]
mod remote_agent;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "systrix=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse CLI arguments
    let cli = cli::Cli::parse();

    // Execute the appropriate command
    cli::execute(cli).await
}
