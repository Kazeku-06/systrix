// src/tui/ui.rs
//! Main UI rendering and layout.

use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::monitor::{CpuSnapshot, DiskSnapshot, MemorySnapshot, MonitorBackend, NetworkSnapshot, ProcessInfo};
use super::panels::{overview, processes};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Dracula,
}

impl Theme {
    pub fn primary_color(&self) -> Color {
        match self {
            Theme::Dark => Color::Cyan,
            Theme::Light => Color::Blue,
            Theme::Dracula => Color::Magenta,
        }
    }
    
    #[allow(dead_code)]
    pub fn bg_color(&self) -> Color {
        match self {
            Theme::Dark => Color::Black,
            Theme::Light => Color::White,
            Theme::Dracula => Color::Rgb(40, 42, 54),
        }
    }
}

pub struct Ui {
    active_panel: usize,
    scroll_offset: usize,
    theme: Theme,
    paused: bool,
    show_modal: bool,
    modal_message: String,
    
    // Data
    cpu_data: Option<CpuSnapshot>,
    memory_data: Option<MemorySnapshot>,
    disk_data: Option<DiskSnapshot>,
    network_data: Option<NetworkSnapshot>,
    process_data: Vec<ProcessInfo>,
    selected_process_index: usize,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            active_panel: 0,
            scroll_offset: 0,
            theme: Theme::Dark,
            paused: false,
            show_modal: false,
            modal_message: String::new(),
            cpu_data: None,
            memory_data: None,
            disk_data: None,
            network_data: None,
            process_data: Vec::new(),
            selected_process_index: 0,
        }
    }

    pub async fn update_data(&mut self, backend: &impl MonitorBackend) -> Result<()> {
        if self.paused {
            return Ok(());
        }
        
        self.cpu_data = Some(backend.cpu_snapshot().await?);
        self.memory_data = Some(backend.memory_snapshot().await?);
        self.disk_data = Some(backend.disk_snapshot().await?);
        self.network_data = Some(backend.network_snapshot().await?);
        self.process_data = backend.process_list(None, "cpu", 100).await?;
        
        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame) {
        let size = f.size();
        
        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Content
                Constraint::Length(1),  // Footer
            ])
            .split(size);
        
        // Render header
        self.render_header(f, chunks[0]);
        
        // Render tabs
        self.render_tabs(f, chunks[1]);
        
        // Render active panel
        self.render_panel(f, chunks[2]);
        
        // Render footer
        self.render_footer(f, chunks[3]);
        
        // Render modal if active
        if self.show_modal {
            self.render_modal(f, size);
        }
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let cpu_usage = self.cpu_data.as_ref().map(|c| c.global_usage).unwrap_or(0.0);
        let mem_usage = self.memory_data.as_ref().map(|m| m.usage_percent).unwrap_or(0.0);
        let disk_usage = self.disk_data.as_ref().map(|d| d.usage_percent).unwrap_or(0.0);
        
        let header_text = format!(
            " SYSTRIX │ CPU: {:>5.1}% │ RAM: {:>5.1}% │ DISK: {:>5.1}% ",
            cpu_usage, mem_usage, disk_usage
        );
        
        let header = Paragraph::new(header_text)
            .style(Style::default()
                .fg(self.theme.primary_color())
                .add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        
        f.render_widget(header, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let titles = vec!["Overview", "Processes", "Network", "Disk", "Settings"];
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.active_panel)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(self.theme.primary_color())
                    .add_modifier(Modifier::BOLD)
            );
        
        f.render_widget(tabs, area);
    }

    fn render_panel(&mut self, f: &mut Frame, area: Rect) {
        match self.active_panel {
            0 => overview::render(f, area, &self.cpu_data, &self.memory_data, &self.disk_data, &self.network_data, &self.theme),
            1 => processes::render(f, area, &self.process_data, self.selected_process_index, self.scroll_offset, &self.theme),
            2 => self.render_network_panel(f, area),
            3 => self.render_disk_panel(f, area),
            _ => self.render_settings_panel(f, area),
        }
    }

    fn render_network_panel(&self, f: &mut Frame, area: Rect) {
        let text = "Network panel - TODO";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Network"));
        f.render_widget(paragraph, area);
    }

    fn render_disk_panel(&self, f: &mut Frame, area: Rect) {
        let text = "Disk panel - TODO";
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Disk"));
        f.render_widget(paragraph, area);
    }

    fn render_settings_panel(&self, f: &mut Frame, area: Rect) {
        let text = format!("Theme: {:?}\nPaused: {}", self.theme, self.paused);
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Settings"));
        f.render_widget(paragraph, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer_text = " [q]Quit [1-5]Panels [↑↓]Navigate [k]Kill [p]Pause [t]Theme ";
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(footer, area);
    }

    fn render_modal(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Confirmation")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
        
        let modal_area = centered_rect(60, 20, area);
        
        let text = Paragraph::new(self.modal_message.as_str())
            .block(block)
            .style(Style::default().fg(Color::White));
        
        f.render_widget(text, modal_area);
    }

    // Navigation methods
    pub fn set_active_panel(&mut self, index: usize) {
        self.active_panel = index.min(4);
        self.scroll_offset = 0;
    }

    pub fn next_panel(&mut self) {
        self.active_panel = (self.active_panel + 1) % 5;
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self) {
        if self.active_panel == 1 && self.selected_process_index > 0 {
            self.selected_process_index -= 1;
            if self.selected_process_index < self.scroll_offset {
                self.scroll_offset = self.selected_process_index;
            }
        }
    }

    pub fn scroll_down(&mut self) {
        if self.active_panel == 1 && self.selected_process_index < self.process_data.len().saturating_sub(1) {
            self.selected_process_index += 1;
        }
    }

    pub fn page_up(&mut self) {
        if self.selected_process_index >= 10 {
            self.selected_process_index -= 10;
        } else {
            self.selected_process_index = 0;
        }
    }

    pub fn page_down(&mut self) {
        self.selected_process_index = (self.selected_process_index + 10).min(self.process_data.len().saturating_sub(1));
    }

    pub fn scroll_to_top(&mut self) {
        self.selected_process_index = 0;
        self.scroll_offset = 0;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.selected_process_index = self.process_data.len().saturating_sub(1);
    }

    pub fn show_details(&mut self) {
        // TODO: Show process details modal
    }

    pub async fn kill_selected_process(&mut self) -> Result<()> {
        if self.active_panel == 1 && !self.process_data.is_empty() {
            if let Some(process) = self.process_data.get(self.selected_process_index) {
                self.modal_message = format!("Kill process {} (PID {})?\nPress 'y' to confirm, ESC to cancel", 
                    process.name, process.pid);
                self.show_modal = true;
            }
        }
        Ok(())
    }

    pub async fn suspend_selected_process(&mut self) -> Result<()> {
        // TODO: Implement suspend
        Ok(())
    }

    pub async fn resume_selected_process(&mut self) -> Result<()> {
        // TODO: Implement resume
        Ok(())
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dracula,
            Theme::Dracula => Theme::Dark,
        };
    }

    pub fn start_search(&mut self) {
        // TODO: Implement search
    }

    pub fn cancel_action(&mut self) {
        self.show_modal = false;
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
