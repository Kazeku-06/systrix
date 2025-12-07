// src/tui/event.rs
//! Event handling for TUI.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use super::ui::Ui;

pub struct EventHandler {
    refresh_interval: u64,
}

impl EventHandler {
    pub fn new(refresh_interval: u64) -> Self {
        Self { refresh_interval }
    }

    pub async fn handle_events(&mut self, ui: &mut Ui) -> Result<bool> {
        // Poll for events with timeout
        if event::poll(Duration::from_millis(self.refresh_interval))? {
            if let Event::Key(key) = event::read()? {
                return self.handle_key_event(key, ui).await;
            }
        }
        
        Ok(false)
    }

    async fn handle_key_event(&self, key: KeyEvent, ui: &mut Ui) -> Result<bool> {
        match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(true),
            
            // Quit with Ctrl+C
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(true);
            }
            
            // Panel navigation (or settings category if in settings panel)
            KeyCode::Char('1') => {
                if ui.is_in_settings_panel() {
                    ui.set_settings_category(0);
                } else {
                    ui.set_active_panel(0);
                }
            },
            KeyCode::Char('2') => {
                if ui.is_in_settings_panel() {
                    ui.set_settings_category(1);
                } else {
                    ui.set_active_panel(1);
                }
            },
            KeyCode::Char('3') => {
                if ui.is_in_settings_panel() {
                    ui.set_settings_category(2);
                } else {
                    ui.set_active_panel(2);
                }
            },
            KeyCode::Char('4') => {
                if ui.is_in_settings_panel() {
                    ui.set_settings_category(3);
                } else {
                    ui.set_active_panel(3);
                }
            },
            KeyCode::Char('5') => {
                if ui.is_in_settings_panel() {
                    ui.set_settings_category(4);
                } else {
                    ui.set_active_panel(4);
                }
            },
            KeyCode::Tab => ui.next_panel(),
            
            // List navigation
            KeyCode::Up => ui.scroll_up(),
            KeyCode::Down => ui.scroll_down(),
            KeyCode::PageUp => ui.page_up(),
            KeyCode::PageDown => ui.page_down(),
            KeyCode::Home => ui.scroll_to_top(),
            KeyCode::End => ui.scroll_to_bottom(),
            
            // Actions
            KeyCode::Enter => ui.show_details(),
            KeyCode::Char('k') if !ui.is_search_mode() => ui.kill_selected_process().await?,
            KeyCode::Char('s') if !ui.is_search_mode() => ui.suspend_selected_process().await?,
            KeyCode::Char('r') if !ui.is_search_mode() => ui.resume_selected_process().await?,
            KeyCode::Char('p') if !ui.is_search_mode() => ui.toggle_pause(),
            KeyCode::Char('t') if !ui.is_search_mode() => ui.toggle_theme(),
            KeyCode::Char('/') if !ui.is_search_mode() => ui.start_search(),
            KeyCode::Esc => ui.cancel_action(),
            
            // Search mode
            KeyCode::Char(c) if ui.is_search_mode() => ui.search_input(c),
            KeyCode::Backspace if ui.is_search_mode() => ui.search_backspace(),
            
            _ => {}
        }
        
        Ok(false)
    }
}
