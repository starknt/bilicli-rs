use napi_derive::napi;

#[cfg(feature = "platform-napi")]
pub mod footer;
pub mod header;
pub mod tabs;

#[napi]
#[derive(Default, PartialEq, Eq)]
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
