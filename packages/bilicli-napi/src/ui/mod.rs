pub mod footer;
pub mod header;
pub mod tabs;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
    Quit,
}

#[derive(Default, PartialEq, Eq, Clone)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}
