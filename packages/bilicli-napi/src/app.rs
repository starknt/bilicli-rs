#![allow(clippy::new_without_default)]

use std::time::Duration;

use crate::{
    ui::{
        footer::Footer, header::Header, helper::centered_rect, tabs::Tabs, AppState, InputMode,
        SliderBarState,
    },
    CliState,
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

#[derive(Debug, Default)]
pub struct App {
    pub input_mode: InputMode,
    header: Header,
    footer: Footer,
    tabs: Tabs,
    pub textarea: TextArea<'static>,
    pub will_send_message: Vec<String>,
}

unsafe impl Send for App {}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        state: &mut CliState,
    ) -> Result<()> {
        let mut interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND));
        let mut events = EventStream::new();

        tokio::select! {
            _ = interval.tick() => self.draw(terminal, state)?,
            Some(Ok(event)) = events.next() =>  self.handle_events(&event, state)?,
        }

        Ok(())
    }

    fn draw(
        &mut self,
        terminal: &mut Terminal<impl Backend>,
        state: &mut CliState,
    ) -> crate::app::Result<()> {
        terminal.draw(|f| f.render_stateful_widget(self, f.area(), state))?;

        Ok(())
    }

    pub fn handle_events(&mut self, event: &Event, state: &mut CliState) -> crate::app::Result<()> {
        if let Event::Key(key) = event {
            match self.input_mode {
                InputMode::Normal => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Up if state.state == AppState::Running => self.previous_tab(),
                            KeyCode::Down if state.state == AppState::Running => self.next_tab(),
                            KeyCode::Char('w') if state.state == AppState::Running => {
                                self.scroll_up()
                            }
                            KeyCode::Char('s') if state.state == AppState::Running => {
                                self.scroll_down()
                            }
                            KeyCode::Char('q') if state.state == AppState::Running => state.quit(),
                            KeyCode::Char('y') if state.state == AppState::Quitting => {
                                state.state = AppState::Quit
                            }
                            KeyCode::Char('n') if state.state == AppState::Quitting => {
                                state.state = AppState::Running;
                            }
                            KeyCode::Char('t') if state.state == AppState::Running => {
                                self.toggle_slider_bar(state)
                            }
                            KeyCode::Enter if state.state == AppState::Running => {
                                self.input_mode = InputMode::Editing;
                                self.textarea
                                    .set_style(Style::default().fg(Color::LightGreen));
                            }
                            _ => {}
                        }
                    }
                }
                InputMode::Editing => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Up => {
                                self.previous_tab();
                            }
                            KeyCode::Down => {
                                self.next_tab();
                            }
                            KeyCode::Enter => {
                                if !self.textarea.lines()[0].is_empty() {
                                    self.will_send_message.push(self.textarea.yank_text());
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
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn toggle_slider_bar(&mut self, state: &mut CliState) {
        state.slider_bar_state = match state.slider_bar_state {
            SliderBarState::Normal => SliderBarState::Hiding,
            SliderBarState::Hiding => SliderBarState::Normal,
        };
    }

    pub fn scroll_up(&mut self) {
        if let Some(index) = self.tabs.state.selected() {
            let tab = &mut self.tabs.tabs[index];
            tab.scroll_up();
        }
    }

    pub fn scroll_down(&mut self) {
        if let Some(index) = self.tabs.state.selected() {
            let tab = &mut self.tabs.tabs[index];
            tab.scroll_down();
        }
    }

    pub fn next_tab(&mut self) {
        self.tabs.next_tab();
    }

    pub fn previous_tab(&mut self) {
        self.tabs.previous_tab();
    }
}

impl StatefulWidget for &mut App {
    type State = CliState;

    fn render(self, root: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.state == AppState::Quitting {
            self.render_quit_question(root, buf);
            return;
        }

        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);

        let horizontal: Layout = {
            if state.slider_bar_state == SliderBarState::Normal {
                Layout::horizontal([Constraint::Length(8), Constraint::Fill(1)])
            } else {
                Layout::horizontal([Constraint::Length(0), Constraint::Fill(1)])
            }
        };

        let [header_area, inner_area, footer_area] = vertical.areas(root);
        let [tabs_area, content_area] = horizontal.areas(inner_area);

        self.header.render(header_area, buf, state);

        if state.slider_bar_state == SliderBarState::Normal {
            self.render_tabs(tabs_area, buf);
        }

        self.render_selected_tab(content_area, buf, state);

        if self.input_mode == InputMode::Editing {
            self.render_input(footer_area, buf);
        } else {
            self.footer.render(footer_area, buf, state);
        }
    }
}

impl App {
    fn render_quit_question(&self, area: Rect, buf: &mut Buffer) {
        let area = centered_rect(60, 40, area);
        let block = Block::bordered()
            .title(" 提示 ")
            .title_alignment(Alignment::Center)
            .border_set(symbols::border::ROUNDED)
            .border_style(Style::default().fg(tailwind::ORANGE.c400))
            .padding(Padding::uniform(1));

        Paragraph::new(vec![
            Line::from("你确定要退出吗?".bold()).centered(),
            Line::raw(""),
            Line::raw(""),
            Line::from(vec![
                Span::from("确定 (按 Y/y)").bold().fg(tailwind::ORANGE.c300),
                Span::raw("    "),
                Span::from("取消 (按 N/n)").bold().fg(tailwind::RED.c300),
            ])
            .centered(),
        ])
        .block(block)
        .wrap(Wrap { trim: true })
        .render(area, buf);
    }

    fn render_tabs(&mut self, area: Rect, buf: &mut Buffer) {
        let highlight_style = Style::default()
            .bg(tailwind::YELLOW.c300)
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
            .highlight_spacing(ratatui::widgets::HighlightSpacing::WhenSelected);

        StatefulWidget::render(list, area, buf, &mut self.tabs.state);
    }

    fn render_selected_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let tab = if let Some(index) = self.tabs.state.selected() {
            &mut self.tabs.tabs[index]
        } else {
            self.tabs.state.select_first();
            &mut self.tabs.tabs[0]
        };

        tab.render(area, buf, state);
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
