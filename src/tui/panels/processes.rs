// src/tui/panels/processes.rs
//! Processes panel showing process list.

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::monitor::ProcessInfo;
use crate::tui::ui::Theme;

pub fn render(
    f: &mut Frame,
    area: Rect,
    processes: &[&ProcessInfo],
    selected_index: usize,
    _scroll_offset: usize,
    theme: &Theme,
    search_query: &str,
    search_mode: bool,
) {
    let header_cells = ["PID", "USER", "NAME", "CPU%", "MEM%", "THREADS"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);
    
    let rows = processes.iter().enumerate().map(|(i, proc)| {
        let style = if i == selected_index {
            Style::default()
                .bg(theme.primary_color())
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        
        let cells = vec![
            Cell::from(proc.pid.to_string()),
            Cell::from(proc.user.chars().take(10).collect::<String>()),
            Cell::from(proc.name.chars().take(20).collect::<String>()),
            Cell::from(format!("{:.1}", proc.cpu_usage)),
            Cell::from(format!("{:.1}", proc.memory_usage)),
            Cell::from(proc.threads.to_string()),
        ];
        
        Row::new(cells).style(style).height(1)
    });
    
    let title = if search_mode {
        format!("Processes - Search: {}█", search_query)
    } else if !search_query.is_empty() {
        format!("Processes ({} filtered) - Press ESC to clear", processes.len())
    } else {
        format!("Processes ({} total) - Press / to search", processes.len())
    };
    
    let table = Table::new(rows, [
        Constraint::Length(8),
        Constraint::Length(12),
        Constraint::Length(22),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(10),
    ])
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .title(title));
    
    f.render_widget(table, area);
}
