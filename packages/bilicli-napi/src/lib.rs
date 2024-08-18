#[cfg(feature = "napi")]
use crate::app::App;
#[cfg(feature = "napi")]
use napi::bindgen_prelude::*;
#[cfg(feature = "napi")]
use napi_derive::napi;
#[cfg(feature = "napi")]
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

pub mod api;
pub mod app;
pub mod ui;

#[cfg(napi)]
#[napi]
#[derive(Clone)]
pub struct Cli {
    pub room_id: u32,
    app: App,
}

#[cfg(napi)]
#[napi]
impl Cli {
    #[napi(constructor)]
    pub fn new(room_id: u32) -> Result<Self> {
        let app = App::new(room_id);

        Ok(Self { room_id, app })
    }

    #[napi]
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        self.app.run(&mut terminal).unwrap();

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }
}
