// src/tui/panels/network.rs
//! Network panel with interface details and traffic graphs.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::monitor::NetworkSnapshot;
use crate::tui::ui::Theme;
use crate::utils;

pub fn render(
    f: &mut Frame,
    area: Rect,
    network_data: &Option<NetworkSnapshot>,
    theme: &Theme,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Summary
            Constraint::Min(0),     // Interface table
        ])
        .split(area);
    
    // Summary
    if let Some(network) = network_data {
        let summary_text = format!(
            "Total: ↓ {} | ↑ {} | {} interfaces",
            utils::format_bytes(network.total_rx),
            utils::format_bytes(network.total_tx),
            network.interfaces.len()
        );
        
        let summary = Paragraph::new(summary_text)
            .style(Style::default().fg(theme.primary_color()).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Network Summary"));
        
        f.render_widget(summary, chunks[0]);
        
        // Interface table
        let header_cells = ["Interface", "RX Total", "TX Total", "RX Rate", "TX Rate", "Packets RX", "Packets TX"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells)
            .style(Style::default().bg(Color::DarkGray))
            .height(1);
        
        let rows = network.interfaces.iter().map(|iface| {
            let cells = vec![
                Cell::from(iface.name.clone()),
                Cell::from(utils::format_bytes(iface.received)),
                Cell::from(utils::format_bytes(iface.transmitted)),
                Cell::from(format!("{}/s", utils::format_bytes(iface.rx_rate))),
                Cell::from(format!("{}/s", utils::format_bytes(iface.tx_rate))),
                Cell::from(iface.packets_received.to_string()),
                Cell::from(iface.packets_transmitted.to_string()),
            ];
            Row::new(cells).height(1)
        });
        
        let table = Table::new(rows, [
            Constraint::Length(15),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
        ])
        .header(header)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Network Interfaces"));
        
        f.render_widget(table, chunks[1]);
    } else {
        let text = Paragraph::new("Loading network data...")
            .block(Block::default().borders(Borders::ALL).title("Network"));
        f.render_widget(text, area);
    }
}
