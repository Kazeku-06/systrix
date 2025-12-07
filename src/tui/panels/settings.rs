// src/tui/panels/settings.rs
//! Settings panel with configuration options.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::ui::{Theme, SettingsState};

pub fn render(
    f: &mut Frame,
    area: Rect,
    settings: &SettingsState,
    theme: &Theme,
    paused: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),  // Settings menu
            Constraint::Percentage(60),  // Settings details
        ])
        .split(area);
    
    // Settings menu
    render_settings_menu(f, chunks[0], settings, theme);
    
    // Settings details
    render_settings_details(f, chunks[1], settings, theme, paused);
}

fn render_settings_menu(
    f: &mut Frame,
    area: Rect,
    settings: &SettingsState,
    theme: &Theme,
) {
    let menu_items = vec![
        "Appearance",
        "Performance",
        "Display",
        "Keyboard",
        "About",
    ];
    
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == settings.selected_category {
                Style::default()
                    .fg(theme.primary_color())
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            } else {
                Style::default()
            };
            
            let prefix = if i == settings.selected_category { "▶ " } else { "  " };
            ListItem::new(format!("{}{}", prefix, item)).style(style)
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Settings Categories"));
    
    f.render_widget(list, area);
}

fn render_settings_details(
    f: &mut Frame,
    area: Rect,
    settings: &SettingsState,
    theme: &Theme,
    paused: bool,
) {
    match settings.selected_category {
        0 => render_appearance_settings(f, area, settings, theme),
        1 => render_performance_settings(f, area, settings, paused),
        2 => render_display_settings(f, area, settings),
        3 => render_keyboard_settings(f, area),
        4 => render_about(f, area, theme),
        _ => {}
    }
}

fn render_appearance_settings(
    f: &mut Frame,
    area: Rect,
    _settings: &SettingsState,
    theme: &Theme,
) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Theme Settings", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Current Theme: "),
            Span::styled(
                format!("{:?}", theme),
                Style::default().fg(theme.primary_color()).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(""),
        Line::from("Available Themes:"),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled("Dark", Style::default().fg(Color::Cyan)),
            Span::raw(" - Default dark theme"),
        ]),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled("Light", Style::default().fg(Color::Blue)),
            Span::raw(" - Light theme for bright environments"),
        ]),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled("Dracula", Style::default().fg(Color::Magenta)),
            Span::raw(" - Popular Dracula color scheme"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press 't' to cycle themes", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from("Color Coding:"),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled("Red", Style::default().fg(Color::Red)),
            Span::raw(" - Critical (>90% usage)"),
        ]),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled("Yellow", Style::default().fg(Color::Yellow)),
            Span::raw(" - Warning (>75% usage)"),
        ]),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled("Green", Style::default().fg(Color::Green)),
            Span::raw(" - Normal (<75% usage)"),
        ]),
    ];
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Appearance"));
    
    f.render_widget(paragraph, area);
}

fn render_performance_settings(
    f: &mut Frame,
    area: Rect,
    settings: &SettingsState,
    paused: bool,
) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Performance Settings", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Refresh Interval: "),
            Span::styled(
                format!("{} ms", settings.refresh_interval),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Status: "),
            if paused {
                Span::styled("⏸ PAUSED", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("▶ RUNNING", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            },
        ]),
        Line::from(""),
        Line::from("Recommendations:"),
        Line::from("  • 100-250ms - High frequency (higher CPU usage)"),
        Line::from("  • 500ms - Default (balanced)"),
        Line::from("  • 1000-2000ms - Low frequency (lower CPU usage)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press 'p' to pause/resume monitoring", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from("Performance Tips:"),
        Line::from("  • Increase refresh interval to reduce CPU usage"),
        Line::from("  • Pause monitoring when not actively viewing"),
        Line::from("  • Close unused panels"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Target CPU Usage: "),
            Span::styled("3-5% idle", Style::default().fg(Color::Green)),
        ]),
    ];
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Performance"));
    
    f.render_widget(paragraph, area);
}

fn render_display_settings(
    f: &mut Frame,
    area: Rect,
    settings: &SettingsState,
) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Display Settings", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Process Limit: "),
            Span::styled(
                format!("{}", settings.process_limit),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Show Graphs: "),
            Span::styled(
                if settings.show_graphs { "✓ Enabled" } else { "✗ Disabled" },
                Style::default().fg(if settings.show_graphs { Color::Green } else { Color::Red })
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Show Per-Core CPU: "),
            Span::styled(
                if settings.show_per_core_cpu { "✓ Enabled" } else { "✗ Disabled" },
                Style::default().fg(if settings.show_per_core_cpu { Color::Green } else { Color::Red })
            ),
        ]),
        Line::from(""),
        Line::from("Panel Options:"),
        Line::from("  1. Overview - System summary with gauges"),
        Line::from("  2. Processes - Process list with search"),
        Line::from("  3. Network - Network interface statistics"),
        Line::from("  4. Disk - Disk partition information"),
        Line::from("  5. Settings - This panel"),
        Line::from(""),
        Line::from("Display Features:"),
        Line::from("  • Real-time updates"),
        Line::from("  • Color-coded warnings"),
        Line::from("  • Sortable columns"),
        Line::from("  • Search and filter"),
    ];
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Display"));
    
    f.render_widget(paragraph, area);
}

fn render_keyboard_settings(
    f: &mut Frame,
    area: Rect,
) {
    let lines = vec![
        Line::from(vec![
            Span::styled("Keyboard Shortcuts", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from("  q - Quit application"),
        Line::from("  1-5 - Switch to panel (Overview, Processes, etc)"),
        Line::from("  Tab - Next panel"),
        Line::from("  ↑↓ - Navigate list"),
        Line::from("  PageUp/PageDown - Scroll by page"),
        Line::from("  Home/End - Jump to top/bottom"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Actions:", Style::default().add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from("  Enter - Show process details"),
        Line::from("  k - Kill selected process"),
        Line::from("  / - Search processes"),
        Line::from("  Esc - Cancel/Clear search"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Settings:", Style::default().add_modifier(Modifier::UNDERLINED)),
        ]),
        Line::from("  p - Pause/Resume monitoring"),
        Line::from("  t - Toggle theme"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tips:", Style::default().fg(Color::Yellow)),
        ]),
        Line::from("  • Use search (/) to quickly find processes"),
        Line::from("  • Pause (p) to examine data without updates"),
        Line::from("  • Press Enter for detailed process info"),
    ];
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Keyboard Shortcuts"));
    
    f.render_widget(paragraph, area);
}

fn render_about(
    f: &mut Frame,
    area: Rect,
    theme: &Theme,
) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "SYSTRIX",
                Style::default()
                    .fg(theme.primary_color())
                    .add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "System Monitor - CLI + TUI",
                Style::default().fg(Color::Gray)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Version: "),
            Span::styled(
                env!("CARGO_PKG_VERSION"),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Built with: "),
            Span::styled("🦀 Rust", Style::default().fg(Color::Red)),
        ]),
        Line::from(""),
        Line::from("Features:"),
        Line::from("  ✓ Real-time system monitoring"),
        Line::from("  ✓ Interactive TUI with multiple panels"),
        Line::from("  ✓ Process management"),
        Line::from("  ✓ Network and disk monitoring"),
        Line::from("  ✓ Multiple themes"),
        Line::from("  ✓ Cross-platform (Linux, macOS, Windows)"),
        Line::from(""),
        Line::from("Technology Stack:"),
        Line::from("  • ratatui - Terminal UI framework"),
        Line::from("  • tokio - Async runtime"),
        Line::from("  • sysinfo - System information"),
        Line::from("  • clap - CLI argument parsing"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Repository: "),
            Span::styled(
                "github.com/Kazeku-06/systrix",
                Style::default().fg(Color::Blue).add_modifier(Modifier::UNDERLINED)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("License: "),
            Span::styled("MIT", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Made by nopalll",
                Style::default().fg(Color::Magenta)
            ),
        ]),
    ];
    
    let paragraph = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("About Systrix"))
        .alignment(ratatui::layout::Alignment::Center);
    
    f.render_widget(paragraph, area);
}
