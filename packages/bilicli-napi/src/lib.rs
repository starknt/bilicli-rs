use std::{
    io::stdout,
    panic::{set_hook, take_hook},
};

#[cfg(feature = "platform-napi")]
use crate::{api::get_room_info, app::App, ui::AppState};

#[cfg(feature = "platform-napi")]
use napi::bindgen_prelude::*;
#[cfg(feature = "platform-napi")]
use napi_derive::napi;

#[cfg(feature = "platform-napi")]
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

#[cfg(not(feature = "platform-napi"))]
use ratatui::crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

pub mod api;
pub mod app;
pub mod ui;

#[cfg(feature = "platform-napi")]
#[napi]
pub struct Cli {
    pub room_id: u32,
    app: App,
}

#[cfg(feature = "platform-napi")]
#[napi]
impl Cli {
    #[napi(constructor)]
    pub fn new(room_id: u32) -> Result<Self> {
        let app = App::new(room_id);

        Ok(Self { room_id, app })
    }

    #[napi(getter)]
    pub fn state(&self) -> AppState {
        self.app.state
    }

    #[napi]
    pub fn run(&mut self) -> Result<()> {
        init_panic_hook();

        let mut app = self.app.clone();
        let room_id = self.room_id;
        let rt = tokio::runtime::Builder::new_multi_thread() // 使用 Builder 创建多线程运行时
            .worker_threads(4) // 设置工作线程数
            .enable_all() // 启用所有功能
            .build() // 构建运行时
            .unwrap();

        rt.block_on(async move {
            let mut stdout = stdout();
            enable_raw_mode().unwrap();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
            let info = get_room_info(room_id).await.unwrap();
            app.update_info(info);
            app.run(&mut terminal).await.unwrap();
        });

        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    #[napi]
    pub fn stop(&mut self) -> Result<()> {
        self.app.state = AppState::Quit;

        Ok(())
    }

    #[napi]
    pub fn send_attention_change(&mut self, attention: u32) {
        self.app.send_attention_change(attention);
    }

    #[napi]
    pub fn send_watcher_change(&mut self, watcher: String) {
        self.app.send_watcher_change(watcher);
    }

    #[napi]
    pub fn send_live_change(&mut self, live: bool) {
        self.app.send_live_change(live);
    }
}

pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));
}
