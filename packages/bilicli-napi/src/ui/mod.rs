#[cfg(feature = "platform-napi")]
use napi_derive::napi;

pub mod footer;
pub mod header;
pub mod tabs;

#[cfg(feature = "platform-napi")]
#[napi]
#[derive(Default, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
    Quit,
}

#[cfg(not(feature = "platform-napi"))]
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
    Quit,
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}
