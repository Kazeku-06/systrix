// src/tui/panels/overview.rs
//! Overview panel showing system summary.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::monitor::{BatteryInfo, CpuSnapshot, DiskSnapshot, MemorySnapshot, NetworkSnapshot};
use crate::tui::ui::Theme;
use crate::utils;

pub fn render(
    f: &mut Frame,
    area: Rect,
    cpu_data: &Option<CpuSnapshot>,
    memory_data: &Option<MemorySnapshot>,
    disk_data: &Option<DiskSnapshot>,
    network_data: &Option<NetworkSnapshot>,
    battery_data: &Option<BatteryInfo>,
    theme: &Theme,
) {
    // Check if battery is present to adjust layout
    let has_battery = battery_data.as_ref().map(|b| b.is_present).unwrap_or(false);
    
    let chunks = if has_battery {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // CPU
                Constraint::Length(3),  // Memory
                Constraint::Length(3),  // Disk
                Constraint::Length(3),  // Network
                Constraint::Length(3),  // Battery
                Constraint::Min(0),     // Details
            ])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // CPU
                Constraint::Length(3),  // Memory
                Constraint::Length(3),  // Disk
                Constraint::Length(3),  // Network
                Constraint::Min(0),     // Details
            ])
            .split(area)
    };
    
    // CPU gauge
    if let Some(cpu) = cpu_data {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
            .gauge_style(Style::default().fg(theme.primary_color()))
            .percent(cpu.global_usage as u16)
            .label(format!("{:.1}%", cpu.global_usage));
        f.render_widget(gauge, chunks[0]);
    }
    
    // Memory gauge
    if let Some(memory) = memory_data {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(memory.usage_percent as u16)
            .label(format!("{:.1}% ({} / {})", 
                memory.usage_percent,
                utils::format_bytes(memory.used),
                utils::format_bytes(memory.total)));
        f.render_widget(gauge, chunks[1]);
    }
    
    // Disk gauge
    if let Some(disk) = disk_data {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Disk Usage"))
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent(disk.usage_percent as u16)
            .label(format!("{:.1}% ({} / {})", 
                disk.usage_percent,
                utils::format_bytes(disk.used),
                utils::format_bytes(disk.total)));
        f.render_widget(gauge, chunks[2]);
    }
    
    // Network info
    if let Some(network) = network_data {
        let net_text = format!(
            "↓ {} | ↑ {}",
            utils::format_bytes(network.total_rx),
            utils::format_bytes(network.total_tx)
        );
        let paragraph = Paragraph::new(net_text)
            .block(Block::default().borders(Borders::ALL).title("Network"));
        f.render_widget(paragraph, chunks[3]);
    }
    
    // Battery gauge (if present)
    if has_battery {
        if let Some(battery) = battery_data {
            let battery_color = crate::monitor::battery::get_battery_color(battery.percentage, battery.is_charging);
            let status_icon = if battery.is_charging { "⚡" } else { "🔋" };
            
            let mut label = format!("{} {:.0}%", status_icon, battery.percentage);
            if let Some(time) = battery.time_remaining {
                label.push_str(&format!(" ({})", crate::monitor::battery::format_time_remaining(time)));
            }
            
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(format!("Battery - {}", battery.status)))
                .gauge_style(Style::default().fg(battery_color))
                .percent(battery.percentage as u16)
                .label(label);
            f.render_widget(gauge, chunks[4]);
        }
    }
    
    // System details
    let mut details_lines = Vec::new();
    
    if let Some(cpu) = cpu_data {
        details_lines.push(Line::from(vec![
            Span::styled("CPU Model: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&cpu.model),
        ]));
        details_lines.push(Line::from(vec![
            Span::styled("Cores: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{} physical, {} logical", cpu.physical_cores, cpu.logical_cores)),
        ]));
        details_lines.push(Line::from(vec![
            Span::styled("Frequency: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{:.0} MHz", cpu.frequency)),
        ]));
        details_lines.push(Line::from(vec![
            Span::styled("Uptime: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(utils::format_duration(cpu.uptime)),
        ]));
        details_lines.push(Line::from(vec![
            Span::styled("OS: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&cpu.os_name),
        ]));
    }
    
    // Add battery details if present
    if has_battery {
        if let Some(battery) = battery_data {
            details_lines.push(Line::from(""));
            details_lines.push(Line::from(vec![
                Span::styled("Battery Health: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:.0}%", battery.health)),
            ]));
            if !battery.vendor.is_empty() && battery.vendor != "Unknown" {
                details_lines.push(Line::from(vec![
                    Span::styled("Battery Vendor: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(&battery.vendor),
                ]));
            }
        }
    }
    
    let details_chunk = if has_battery { chunks[5] } else { chunks[4] };
    let details = Paragraph::new(details_lines)
        .block(Block::default().borders(Borders::ALL).title("System Information"));
    f.render_widget(details, details_chunk);
}
