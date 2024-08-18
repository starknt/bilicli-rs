#![allow(clippy::new_without_default)]

use std::time::Duration;

use crate::{
    api::RoomInfo,
    ui::{footer::Footer, header::Header, tabs::Tabs, AppState, InputMode},
};
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
};
use tui_textarea::TextArea;

pub const MAX_INPUT_LENGTH: usize = 40;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct App {
    pub input_mode: InputMode,
    pub state: AppState,
    header: Header,
    footer: Footer,
    tabs: Tabs,
    pub textarea: TextArea<'static>,
    pub messages: Vec<String>,
}

unsafe impl Send for App {}

impl App {
    pub fn new(_: u32) -> Self {
        Self {
            textarea: TextArea::default(),
            input_mode: InputMode::Normal,
            state: AppState::Running,
            header: Header::default(),
            footer: Footer::default(),
            tabs: Tabs::default(),
            messages: vec![],
        }
    }

    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        let mut interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND));
        let mut events = EventStream::new();

        while self.state != AppState::Quit {
            tokio::select! {
                _ = interval.tick() => self.draw(terminal)?,
                Some(Ok(event)) = events.next() =>  self.handle_events(&event)?,
            }
        }

        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> crate::app::Result<()> {
        terminal.draw(|f| {
            f.render_widget(self, f.area());
        })?;

        Ok(())
    }

    fn handle_events(&mut self, event: &Event) -> crate::app::Result<()> {
        if let Event::Key(key) = event {
            match self.input_mode {
                InputMode::Normal => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Up => self.previous_tab(),
                            KeyCode::Down => self.next_tab(),
                            KeyCode::Char('w') => {
                                self.scroll_up();
                            }
                            KeyCode::Char('s') => {
                                self.scroll_down();
                            }
                            KeyCode::Char('q') => self.quit(),
                            KeyCode::Char('y') => {
                                if self.state == AppState::Quitting {
                                    self.state = AppState::Quit;
                                }
                            }
                            KeyCode::Char('n') => {
                                if self.state == AppState::Quitting {
                                    self.state = AppState::Running;
                                }
                            }
                            KeyCode::Enter => {
                                self.input_mode = InputMode::Editing;
                                self.textarea
                                    .set_style(Style::default().fg(Color::LightGreen));
                            }
                            _ => {}
                        }
                    }
                }
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        if !self.textarea.lines()[0].is_empty() {
                            self.messages.push(self.textarea.yank_text());
                            self.textarea.delete_str(self.textarea.lines()[0].len());
                        }
                    }
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                    }
                    _ => {
                        if self.textarea.input(*key)
                            && self.textarea.lines()[0].len() > MAX_INPUT_LENGTH
                        {
                            self.textarea.delete_char();
                        }
                    }
                },
            }
        }
        Ok(())
    }

    pub fn update_info(&mut self, info: RoomInfo) {
        self.header.update_info(info.clone());
        self.footer.update_info(info.clone());
    }

    pub fn scroll_up(&mut self) {
        let tab = &mut self.tabs.tabs[self.tabs.state.selected().unwrap()];
        tab.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        let tab = &mut self.tabs.tabs[self.tabs.state.selected().unwrap()];
        tab.scroll_down();
    }

    pub fn next_tab(&mut self) {
        self.tabs.state.select_next();
    }

    pub fn previous_tab(&mut self) {
        self.tabs.state.select_previous();
    }

    pub fn send_attention_change(&mut self, attention: u32) {
        self.footer.update_attention(attention);
    }

    pub fn send_watcher_change(&mut self, watched: String) {
        self.footer.update_watcher(watched);
    }

    pub fn send_live_change(&mut self, live: bool) {
        self.footer.update_live(live);
    }

    pub fn quit(&mut self) {
        if self.state == AppState::Quit {
            return;
        }

        if self.state == AppState::Quitting {
            self.state = AppState::Quit;
            return;
        }

        self.state = AppState::Quitting;
    }
}

impl Widget for &mut App {
    fn render(self, root: Rect, buf: &mut Buffer) {
        if self.state == AppState::Quitting {
            self.render_quit_question(root, buf);
            return;
        }

        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);

        let horizontal: Layout = Layout::horizontal([Constraint::Length(8), Constraint::Fill(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(root);
        let [tabs_area, content_area] = horizontal.areas(inner_area);
        self.header.render(header_area, buf);
        self.render_tabs(tabs_area, buf);
        self.render_selected_tab(content_area, buf);
        if self.input_mode == InputMode::Editing {
            self.render_input(footer_area, buf);
        } else {
            self.footer.render(footer_area, buf);
        }
    }
}

impl App {
    fn render_quit_question(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Quit?").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .padding(Padding::horizontal(1));

        Paragraph::new("Are you sure you want to quit?(Y/N)".bold())
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    fn render_tabs(&mut self, area: Rect, buf: &mut Buffer) {
        let highlight_style = Style::default()
            .bg(tailwind::YELLOW.c400)
            .fg(tailwind::BLACK)
            .bold();
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1));

        let tabs: Vec<ListItem> = self
            .tabs
            .tabs
            .iter()
            .map(|tab| ListItem::from(tab.title()))
            .collect();

        let list = List::new(tabs)
            .block(block)
            .highlight_style(highlight_style)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.tabs.state);
    }

    fn render_selected_tab(&mut self, area: Rect, buf: &mut Buffer) {
        let tab = &mut self.tabs.tabs[self.tabs.state.selected().unwrap()];
        tab.render(area, buf);
    }

    fn render_input(&mut self, area: Rect, buf: &mut Buffer) {
        self.textarea
            .set_placeholder_text("按 Enter 发送弹幕, Esc 取消输入");
        self.textarea
            .set_style(Style::default().fg(Color::LightCyan));
        let style = {
            if !self.textarea.lines()[0].is_empty() {
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::default())
                    .bold()
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            }
        };
        self.textarea.set_cursor_line_style(style);
        self.textarea.set_block(
            Block::default()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Color::LightGreen)
                .borders(Borders::ALL)
                .padding(Padding::left(1))
                .title(format!(
                    " {} / {} ",
                    self.textarea.lines()[0].len(),
                    MAX_INPUT_LENGTH
                ))
                .title_alignment(Alignment::Center),
        );

        self.textarea.render(area, buf);
    }
}
