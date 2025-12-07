// src/tui/ui.rs
//! Main UI rendering and layout.

use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::monitor::{CpuSnapshot, DiskInfo, DiskSnapshot, MemorySnapshot, MonitorBackend, NetworkSnapshot, ProcessInfo};
use super::panels::{disk, network, overview, processes, settings};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Dracula,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalType {
    None,
    KillConfirm,
    ProcessDetail,
}

pub struct SettingsState {
    pub selected_category: usize,
    pub refresh_interval: u64,
    pub process_limit: usize,
    pub show_graphs: bool,
    pub show_per_core_cpu: bool,
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
    modal_type: ModalType,
    search_mode: bool,
    search_query: String,
    settings_state: SettingsState,
    
    // Data
    cpu_data: Option<CpuSnapshot>,
    memory_data: Option<MemorySnapshot>,
    disk_data: Option<DiskSnapshot>,
    network_data: Option<NetworkSnapshot>,
    process_data: Vec<ProcessInfo>,
    disk_list: Vec<DiskInfo>,
    selected_process_index: usize,
    filtered_process_indices: Vec<usize>,
}

impl Ui {
    pub fn with_refresh_interval(refresh_interval: u64) -> Self {
        Self {
            active_panel: 0,
            scroll_offset: 0,
            theme: Theme::Dark,
            paused: false,
            show_modal: false,
            modal_message: String::new(),
            modal_type: ModalType::None,
            search_mode: false,
            search_query: String::new(),
            settings_state: SettingsState {
                selected_category: 0,
                refresh_interval,
                process_limit: 100,
                show_graphs: true,
                show_per_core_cpu: true,
            },
            cpu_data: None,
            memory_data: None,
            disk_data: None,
            network_data: None,
            process_data: Vec::new(),
            disk_list: Vec::new(),
            selected_process_index: 0,
            filtered_process_indices: Vec::new(),
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
        self.disk_list = backend.disk_list().await?;
        
        // Update filtered indices based on search
        self.update_filtered_processes();
        
        Ok(())
    }
    
    fn update_filtered_processes(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_process_indices = (0..self.process_data.len()).collect();
        } else {
            self.filtered_process_indices = self.process_data
                .iter()
                .enumerate()
                .filter(|(_, p)| {
                    p.name.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    p.user.to_lowercase().contains(&self.search_query.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect();
        }
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
            1 => {
                let filtered_processes: Vec<&ProcessInfo> = self.filtered_process_indices
                    .iter()
                    .filter_map(|&i| self.process_data.get(i))
                    .collect();
                
                let actual_selected = if !self.filtered_process_indices.is_empty() {
                    self.selected_process_index.min(self.filtered_process_indices.len() - 1)
                } else {
                    0
                };
                
                processes::render(f, area, &filtered_processes, actual_selected, self.scroll_offset, &self.theme, &self.search_query, self.search_mode);
            },
            2 => network::render(f, area, &self.network_data, &self.theme),
            3 => disk::render(f, area, &self.disk_list, &self.theme),
            _ => settings::render(f, area, &self.settings_state, &self.theme, self.paused),
        }
    }



    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer_text = " [q]Quit [1-5]Panels [↑↓]Navigate [k]Kill [p]Pause [t]Theme ";
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(footer, area);
    }

    fn render_modal(&self, f: &mut Frame, area: Rect) {
        let title = match self.modal_type {
            ModalType::KillConfirm => "Confirmation",
            ModalType::ProcessDetail => "Process Details",
            ModalType::None => "Modal",
        };
        
        let (width, height) = match self.modal_type {
            ModalType::ProcessDetail => (70, 60),
            _ => (60, 20),
        };
        
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
        
        let modal_area = centered_rect(width, height, area);
        
        let text = Paragraph::new(self.modal_message.as_str())
            .block(block)
            .style(Style::default().fg(Color::White))
            .wrap(ratatui::widgets::Wrap { trim: true });
        
        f.render_widget(text, modal_area);
    }

    // Navigation methods
    pub fn set_active_panel(&mut self, index: usize) {
        self.active_panel = index.min(4);
        self.scroll_offset = 0;
        
        // Reset settings category when entering settings panel
        if self.active_panel == 4 {
            self.settings_state.selected_category = 0;
        }
    }
    
    pub fn set_settings_category(&mut self, category: usize) {
        if self.active_panel == 4 {
            self.settings_state.selected_category = category.min(4);
        }
    }

    pub fn next_panel(&mut self) {
        self.active_panel = (self.active_panel + 1) % 5;
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self) {
        match self.active_panel {
            1 => {
                // Processes panel - use filtered list length
                if self.selected_process_index > 0 {
                    self.selected_process_index -= 1;
                    if self.selected_process_index < self.scroll_offset {
                        self.scroll_offset = self.selected_process_index;
                    }
                }
            }
            4 => {
                // Settings panel
                if self.settings_state.selected_category > 0 {
                    self.settings_state.selected_category -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn scroll_down(&mut self) {
        match self.active_panel {
            1 => {
                // Processes panel - use filtered list length
                let max_index = if self.filtered_process_indices.is_empty() {
                    0
                } else {
                    self.filtered_process_indices.len().saturating_sub(1)
                };
                
                if self.selected_process_index < max_index {
                    self.selected_process_index += 1;
                }
            }
            4 => {
                // Settings panel (5 categories: 0-4)
                if self.settings_state.selected_category < 4 {
                    self.settings_state.selected_category += 1;
                }
            }
            _ => {}
        }
    }

    pub fn page_up(&mut self) {
        match self.active_panel {
            1 => {
                // Processes panel - use filtered list
                if self.selected_process_index >= 10 {
                    self.selected_process_index -= 10;
                } else {
                    self.selected_process_index = 0;
                }
            }
            4 => {
                // Settings panel - jump to first category
                self.settings_state.selected_category = 0;
            }
            _ => {}
        }
    }

    pub fn page_down(&mut self) {
        match self.active_panel {
            1 => {
                // Processes panel - use filtered list length
                let max_index = if self.filtered_process_indices.is_empty() {
                    0
                } else {
                    self.filtered_process_indices.len().saturating_sub(1)
                };
                
                self.selected_process_index = (self.selected_process_index + 10).min(max_index);
            }
            4 => {
                // Settings panel - jump to last category
                self.settings_state.selected_category = 4;
            }
            _ => {}
        }
    }

    pub fn scroll_to_top(&mut self) {
        match self.active_panel {
            1 => {
                self.selected_process_index = 0;
                self.scroll_offset = 0;
            }
            4 => {
                self.settings_state.selected_category = 0;
            }
            _ => {}
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        match self.active_panel {
            1 => {
                // Processes panel - use filtered list length
                let max_index = if self.filtered_process_indices.is_empty() {
                    0
                } else {
                    self.filtered_process_indices.len().saturating_sub(1)
                };
                
                self.selected_process_index = max_index;
            }
            4 => {
                self.settings_state.selected_category = 4;
            }
            _ => {}
        }
    }

    pub fn show_details(&mut self) {
        if self.active_panel == 1 && !self.filtered_process_indices.is_empty() {
            let actual_index = self.filtered_process_indices.get(self.selected_process_index);
            if let Some(&idx) = actual_index {
                if let Some(process) = self.process_data.get(idx) {
                    self.modal_message = format!(
                        "Process Details\n\n\
                        PID: {}\n\
                        Name: {}\n\
                        User: {}\n\
                        CPU: {:.1}%\n\
                        Memory: {:.1}%\n\
                        Disk Read: {}\n\
                        Disk Write: {}\n\
                        Threads: {}\n\
                        Status: {}\n\
                        Executable: {}\n\n\
                        Press ESC to close",
                        process.pid,
                        process.name,
                        process.user,
                        process.cpu_usage,
                        process.memory_usage,
                        crate::utils::format_bytes(process.disk_read),
                        crate::utils::format_bytes(process.disk_write),
                        process.threads,
                        process.status,
                        process.exe_path
                    );
                    self.modal_type = ModalType::ProcessDetail;
                    self.show_modal = true;
                }
            }
        }
    }

    pub async fn kill_selected_process(&mut self) -> Result<()> {
        if self.active_panel == 1 && !self.filtered_process_indices.is_empty() {
            let actual_index = self.filtered_process_indices.get(self.selected_process_index);
            if let Some(&idx) = actual_index {
                if let Some(process) = self.process_data.get(idx) {
                    self.modal_message = format!(
                        "Kill process {} (PID {})?\n\nPress 'y' to confirm, ESC to cancel", 
                        process.name, process.pid
                    );
                    self.modal_type = ModalType::KillConfirm;
                    self.show_modal = true;
                }
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
        if self.active_panel == 1 { // Only in Processes panel
            self.search_mode = true;
            self.search_query.clear();
        }
    }

    pub fn cancel_action(&mut self) {
        if self.search_mode {
            self.search_mode = false;
            self.search_query.clear();
            self.update_filtered_processes();
        } else {
            self.show_modal = false;
            self.modal_type = ModalType::None;
        }
    }
    
    pub fn is_search_mode(&self) -> bool {
        self.search_mode
    }
    
    pub fn is_in_settings_panel(&self) -> bool {
        self.active_panel == 4
    }
    
    pub fn search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filtered_processes();
        self.selected_process_index = 0;
    }
    
    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.update_filtered_processes();
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
