// src/app.rs
//! TUI application state and main loop.

#[cfg(feature = "tui")]
use anyhow::Result;

#[cfg(feature = "tui")]
use crate::tui::{event::EventHandler, ui::Ui};
#[cfg(feature = "tui")]
use crate::monitor::SysinfoBackend;
#[cfg(feature = "tui")]
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(feature = "tui")]
use ratatui::{backend::CrosstermBackend, Terminal};
#[cfg(feature = "tui")]
use std::io;

#[cfg(feature = "tui")]
pub struct App {
    refresh_interval: u64,
    backend: SysinfoBackend,
}

#[cfg(feature = "tui")]
impl App {
    pub fn new(refresh_interval: u64) -> Result<Self> {
        Ok(Self {
            refresh_interval,
            backend: SysinfoBackend::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Create UI and event handler
        let mut ui = Ui::with_refresh_interval(self.refresh_interval);
        let mut event_handler = EventHandler::new(self.refresh_interval);

        // Run the app
        let result = self.run_loop(&mut terminal, &mut ui, &mut event_handler).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        ui: &mut Ui,
        event_handler: &mut EventHandler,
    ) -> Result<()> {
        loop {
            // Update data
            ui.update_data(&self.backend).await?;

            // Draw UI
            terminal.draw(|f| ui.render(f))?;

            // Handle events
            if event_handler.handle_events(ui).await? {
                break; // Exit requested
            }
        }

        Ok(())
    }
}
