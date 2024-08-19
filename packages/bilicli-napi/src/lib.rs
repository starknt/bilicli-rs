use std::{
    io::stdout,
    panic::{set_hook, take_hook},
    sync::Arc,
};

#[cfg(feature = "platform-napi")]
use crate::{
    api::get_room_info,
    app::App,
    ui::{AppState, MsgType},
};

use api::RoomInfo;
use chrono::NaiveDateTime;
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
use tokio::sync::Mutex;

pub mod api;
pub mod app;
pub mod ui;

#[cfg(feature = "platform-napi")]
#[napi]
pub struct Cli {
    app: Arc<Mutex<App>>,
    state: Arc<Mutex<CliState>>,
}

#[derive(Clone, Default, Debug)]
pub struct CliState {
    pub state: AppState,
    pub room_id: u32,
    pub attention: u32,
    pub watchers: String,
    pub is_live: bool,
    pub start_time: NaiveDateTime,
    pub area_name: String,
    pub parent_area_name: String,
    pub title: String,
    pub messages: Vec<(MsgType, String)>,
}

#[cfg(feature = "platform-napi")]
#[napi]
impl Cli {
    #[napi(constructor)]
    pub fn new(room_id: u32) -> Result<Self> {
        let app = App::new(room_id);

        Ok(Self {
            app: Arc::new(Mutex::new(app)),
            state: Arc::new(Mutex::new(CliState {
                room_id,
                ..Default::default()
            })),
        })
    }

    #[napi(getter)]
    pub async fn state(&self) -> AppState {
        self.state.lock().await.state
    }

    #[napi]
    pub async unsafe fn run(&mut self) -> Result<()> {
        init_panic_hook();

        let state = Arc::clone(&self.state);
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();

        rt.spawn(async move {
            let mut state = state.lock().await;
            let info = get_room_info(state.room_id).await.unwrap();
            state.update_info(info);
        });

        let state = Arc::clone(&self.state);
        let app = Arc::clone(&self.app);

        rt.spawn(async move {
            let mut stdout = stdout();
            enable_raw_mode().unwrap();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
            let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();

            while state.lock().await.state != AppState::Quitting {
                let mut state = state.lock().await;
                let mut app = app.lock().await;
                app.run(&mut terminal, &mut state).await.unwrap();
                drop(app);
                drop(state);
                std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / 120.0));
            }
        })
        .await
        .unwrap();

        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    #[napi]
    pub async unsafe fn stop(&mut self) -> Result<()> {
        let mut state = self.state.lock().await;
        state.state = AppState::Quit;

        Ok(())
    }

    #[napi]
    pub async unsafe fn send_attention_change(&mut self, attention: u32) {
        let mut state = self.state.lock().await;
        state.update_attention(attention);
    }

    #[napi]
    pub async unsafe fn send_watcher_change(&mut self, watcher: String) {
        let mut state = self.state.lock().await;
        state.update_watcher(watcher);
    }

    #[napi]
    pub async unsafe fn send_live_change(&mut self, live: bool) {
        let mut state = self.state.lock().await;
        state.update_live(live);
    }

    #[napi]
    pub async unsafe fn send_msg(&mut self, t: MsgType, msg: String) {
        let mut state = self.state.lock().await;
        state.messages.push((t, msg));
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

impl CliState {
    pub fn update_info(&mut self, info: RoomInfo) {
        self.area_name = info.area_name;
        self.parent_area_name = info.parent_area_name;
        self.title = info.title;
        self.attention = info.attention;
        self.is_live = info.live_status == 1;
        self.start_time =
            NaiveDateTime::parse_from_str(&info.live_time, "%Y-%m-%d %H:%M:%S").unwrap_or_default();
        self.watchers = info.online.to_string();
    }

    pub fn update_attention(&mut self, attention: u32) {
        self.attention = attention;
    }

    pub fn update_watcher(&mut self, watcher: String) {
        self.watchers = watcher;
    }

    pub fn update_live(&mut self, live: bool) {
        self.is_live = live;
    }
}
