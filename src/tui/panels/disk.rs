// src/tui/panels/disk.rs
//! Disk panel with partition details and usage information.

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::monitor::DiskInfo;
use crate::tui::ui::Theme;
use crate::utils;

pub fn render(
    f: &mut Frame,
    area: Rect,
    disk_list: &[DiskInfo],
    _theme: &Theme,
) {
    if disk_list.is_empty() {
        let text = ratatui::widgets::Paragraph::new("No disk information available")
            .block(Block::default().borders(Borders::ALL).title("Disk"));
        f.render_widget(text, area);
        return;
    }
    
    let header_cells = ["Mount Point", "Type", "Total", "Used", "Available", "Usage"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);
    
    let rows = disk_list.iter().map(|disk| {
        let usage_bar = if disk.total > 0 {
            let percent = (disk.usage_percent as u16).min(100);
            let bar_width = 20;
            let filled = (percent as usize * bar_width / 100).min(bar_width);
            let empty = bar_width - filled;
            format!("[{}{}] {:.1}%", 
                "█".repeat(filled),
                "░".repeat(empty),
                disk.usage_percent)
        } else {
            "N/A".to_string()
        };
        
        let cells = vec![
            Cell::from(disk.mount_point.chars().take(20).collect::<String>()),
            Cell::from(disk.fs_type.chars().take(8).collect::<String>()),
            Cell::from(utils::format_bytes(disk.total)),
            Cell::from(utils::format_bytes(disk.used)),
            Cell::from(utils::format_bytes(disk.available)),
            Cell::from(usage_bar),
        ];
        
        let style = if disk.usage_percent > 90.0 {
            Style::default().fg(Color::Red)
        } else if disk.usage_percent > 75.0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        
        Row::new(cells).height(1).style(style)
    });
    
    let table = Table::new(rows, [
        Constraint::Length(22),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Min(30),
    ])
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .title(format!("Disk Partitions ({} total)", disk_list.len())));
    
    f.render_widget(table, area);
}
