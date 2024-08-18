use chrono::Local;
use ratatui::{
    prelude::*,
    widgets::{Block, ListState, Padding, Paragraph, Scrollbar, ScrollbarState},
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum MsgType {
    #[strum(to_string = "弹幕")]
    Danmu,
    #[strum(to_string = "SC")]
    SuperChat,
    #[strum(to_string = "礼物")]
    Gift,
    #[strum(to_string = "上舰")]
    GuardBuy,
    #[strum(to_string = "进场")]
    InteractWord,
    #[strum(to_string = "进场")]
    EntryEffect,
}

#[derive(Clone, Copy)]
pub enum Tab {
    AllTab(u16, ScrollbarState),
    DanMuTab(u16, ScrollbarState),
    SCTab(u16, ScrollbarState),
    GiftTab(u16, ScrollbarState),
    CaptainTab(u16, ScrollbarState),
    EnterTab(u16, ScrollbarState),
}

impl Tab {
    pub fn title(&self) -> String {
        match self {
            Tab::AllTab(_, _) => "全部".to_string(),
            Tab::DanMuTab(_, _) => "弹幕".to_string(),
            Tab::SCTab(_, _) => "SC".to_string(),
            Tab::GiftTab(_, _) => "礼物".to_string(),
            Tab::CaptainTab(_, _) => "上舰".to_string(),
            Tab::EnterTab(_, _) => "入场".to_string(),
        }
    }

    pub fn scroll_up(&mut self) {
        match self {
            Tab::AllTab(scroll, _) => *scroll = scroll.saturating_sub(1),
            Tab::DanMuTab(scroll, _) => *scroll = scroll.saturating_sub(1),
            Tab::SCTab(scroll, _) => *scroll = scroll.saturating_sub(1),
            Tab::GiftTab(scroll, _) => *scroll = scroll.saturating_sub(1),
            Tab::CaptainTab(scroll, _) => *scroll = scroll.saturating_sub(1),
            Tab::EnterTab(scroll, _) => *scroll = scroll.saturating_sub(1),
        }
    }

    pub fn scroll_down(&mut self) {
        match self {
            Tab::AllTab(scroll, _) => *scroll = scroll.saturating_add(1),
            Tab::DanMuTab(scroll, _) => *scroll = scroll.saturating_add(1),
            Tab::SCTab(scroll, _) => *scroll = scroll.saturating_add(1),
            Tab::GiftTab(scroll, _) => *scroll = scroll.saturating_add(1),
            Tab::CaptainTab(scroll, _) => *scroll = scroll.saturating_add(1),
            Tab::EnterTab(scroll, _) => *scroll = scroll.saturating_add(1),
        }
    }

    pub fn scroll(&self) -> u16 {
        match self {
            Tab::AllTab(scroll, _) => *scroll,
            Tab::DanMuTab(scroll, _) => *scroll,
            Tab::SCTab(scroll, _) => *scroll,
            Tab::GiftTab(scroll, _) => *scroll,
            Tab::CaptainTab(scroll, _) => *scroll,
            Tab::EnterTab(scroll, _) => *scroll,
        }
    }

    pub fn set_state_content_length(&mut self, content_length: usize) {
        match self {
            Tab::AllTab(_, state) => *state = state.content_length(content_length),
            Tab::DanMuTab(_, state) => *state = state.content_length(content_length),
            Tab::SCTab(_, state) => *state = state.content_length(content_length),
            Tab::GiftTab(_, state) => *state = state.content_length(content_length),
            Tab::CaptainTab(_, state) => *state = state.content_length(content_length),
            Tab::EnterTab(_, state) => *state = state.content_length(content_length),
        }
    }

    pub fn state(&mut self) -> &mut ScrollbarState {
        match self {
            Tab::AllTab(_, state) => state,
            Tab::DanMuTab(_, state) => state,
            Tab::SCTab(_, state) => state,
            Tab::GiftTab(_, state) => state,
            Tab::CaptainTab(_, state) => state,
            Tab::EnterTab(_, state) => state,
        }
    }
}

impl Widget for &mut Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Tab::AllTab(_, _) => self.render_all_tab(area, buf),
            Tab::DanMuTab(_, _) => self.render_danmu_tab(area, buf),
            Tab::SCTab(_, _) => self.render_sc_tab(area, buf),
            Tab::GiftTab(_, _) => self.render_gift_tab(area, buf),
            Tab::CaptainTab(_, _) => self.render_captain_tab(area, buf),
            Tab::EnterTab(_, _) => self.render_enter_tab(area, buf),
        }
    }
}

impl Tab {
    fn render_all_tab(&mut self, area: Rect, buf: &mut Buffer) {
        let mut text = vec![
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
            Self::render_msg(),
        ];

        for _ in 0..text.len() {
            text.push(Self::render_msg());
        }

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)), // .border_style(self.palette().c700),
            )
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_danmu_tab(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)), // .border_style(self.palette().c700),
            )
            .render(area, buf);
    }

    fn render_sc_tab(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)), // .border_style(self.palette().c700),
            )
            .render(area, buf);
    }

    fn render_gift_tab(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)), // .border_style(self.palette().c700),
            )
            .render(area, buf);
    }

    fn render_captain_tab(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, Captain!")
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)), // .border_style(self.palette().c700),
            )
            .render(area, buf);
    }

    fn render_enter_tab(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, Enter User!")
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)), // .border_style(self.palette().c700),
            )
            .render(area, buf);
    }
}

impl Tab {
    fn render_msg() -> Line<'static> {
        Line::from(vec![
            Span::from(format!("{}", Local::now().format("%H:%M:%S")))
                .fg(Color::from_hsl(0.0, 0.0, 40.0)),
            Span::raw(" "),
            Span::from(format!("[{}]", MsgType::Danmu)).fg(Color::Yellow),
            Span::raw(" "),
            Span::from("Hello, World!"),
        ])
    }
}

#[derive(Clone)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub state: ListState,
}

impl Default for Tabs {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            tabs: vec![
                Tab::AllTab(0, ScrollbarState::default()),
                Tab::DanMuTab(0, ScrollbarState::default()),
                Tab::SCTab(0, ScrollbarState::default()),
                Tab::GiftTab(0, ScrollbarState::default()),
                Tab::CaptainTab(0, ScrollbarState::default()),
                Tab::EnterTab(0, ScrollbarState::default()),
            ],
            state,
        }
    }
}
