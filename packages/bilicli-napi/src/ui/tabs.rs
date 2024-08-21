use std::{fmt, str::FromStr};

use chrono::{DateTime, Local, TimeZone};
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{block::Title, Block, ListState, Padding, Paragraph, Scrollbar, ScrollbarState},
};

use crate::CliState;

use super::{
    colors::USER_COLORS, DanmuMsg, GiftMsg, GuardBuyMsg, MsgType, SliderBarState, SuperChatMsg,
    UserActionMsg,
};

#[derive(Clone)]
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
            Tab::SCTab(_, _) => "SC  ".to_string(),
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

    pub fn set_state_viewport_content_length(&mut self, viewport_content_length: usize) {
        match self {
            Tab::AllTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::DanMuTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::SCTab(_, state) => *state = state.viewport_content_length(viewport_content_length),
            Tab::GiftTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::CaptainTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
            Tab::EnterTab(_, state) => {
                *state = state.viewport_content_length(viewport_content_length)
            }
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

impl StatefulWidget for &mut Tab {
    type State = CliState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        match self {
            Tab::AllTab(_, _) => self.render_all_tab(area, buf, state),
            Tab::DanMuTab(_, _) => self.render_danmu_tab(area, buf, state),
            Tab::SCTab(_, _) => self.render_sc_tab(area, buf, state),
            Tab::GiftTab(_, _) => self.render_gift_tab(area, buf, state),
            Tab::CaptainTab(_, _) => self.render_captain_tab(area, buf, state),
            Tab::EnterTab(_, _) => self.render_enter_tab(area, buf, state),
        }
    }
}

impl Tab {
    fn should_scroll_down(&mut self, content_length: usize, area: &Rect) -> bool {
        self.scroll() + area.height - 2 < content_length as u16
    }

    fn should_scroll_up(&mut self, area: &Rect, content_length: usize) -> bool {
        self.scroll() > 0 && area.height > content_length as u16
    }

    fn block(&self, state: &CliState) -> Block {
        Block::bordered()
            .title({
                if state.slider_bar_state == SliderBarState::Hiding {
                    Title::from(self.title()).alignment(Alignment::Center)
                } else {
                    Title::from("")
                }
            })
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1))
    }

    fn render_all_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t != MsgType::UserAction)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), true))
            .collect();

        let content_length = {
            let len = text.len();

            if len > area.height as usize {
                len - area.height as usize
            } else {
                0
            }
        };

        self.set_state_content_length(content_length);

        while self.should_scroll_down(text.len(), &area) {
            self.scroll_down();
        }

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_danmu_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::Danmu)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        let content_length = {
            let len = text.len();

            if len > area.height as usize {
                len - area.height as usize
            } else {
                0
            }
        };

        self.set_state_content_length(content_length);

        while self.should_scroll_down(text.len(), &area) {
            self.scroll_down();
        }

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_sc_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::SuperChat)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        let content_length = {
            let len = text.len();

            if len > area.height as usize {
                len - area.height as usize
            } else {
                0
            }
        };

        self.set_state_content_length(content_length);

        while self.should_scroll_down(text.len(), &area) {
            self.scroll_down();
        }

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_gift_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::Gift)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        let content_length = {
            let len = text.len();

            if len > area.height as usize {
                len - area.height as usize
            } else {
                0
            }
        };

        self.set_state_content_length(content_length);

        while self.should_scroll_down(text.len(), &area) {
            self.scroll_down();
        }

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_captain_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::GuardBuy)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        let content_length = {
            let len = text.len();

            if len > area.height as usize {
                len - area.height as usize
            } else {
                0
            }
        };

        self.set_state_content_length(content_length);

        while self.should_scroll_down(text.len(), &area) {
            self.scroll_down();
        }

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }

    fn render_enter_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .filter(|(t, _)| *t == MsgType::UserAction)
            .map(|(t, b)| Self::render_msg(*t, b.clone(), false))
            .collect();

        let content_length = {
            let len = text.len();

            if len > area.height as usize {
                len - area.height as usize
            } else {
                0
            }
        };

        self.set_state_content_length(content_length);

        while self.should_scroll_down(text.len(), &area) {
            self.scroll_down();
        }

        Paragraph::new(text)
            .block(self.block(state))
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }
}

impl Tab {
    fn render_msg(t: MsgType, b: String, render_type: bool) -> Line<'static> {
        match t {
            MsgType::Danmu => {
                let msg = serde_json::from_str::<DanmuMsg>(&b).unwrap();
                let time = get_local_time_from_timestamp(msg.timestamp);

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    {
                        if render_type {
                            Span::from(format!(" [{}] ", t))
                                .fg(Color::LightYellow)
                                .not_bold()
                        } else {
                            Span::raw(" ")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.name)).bg(color)
                        } else {
                            Span::raw("")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.level))
                                .fg(color)
                                .bg(Color::White)
                        } else {
                            Span::raw("")
                        }
                    },
                    Span::raw(" "),
                    {
                        let color = {
                            if let Some(identity) = msg.user.identity {
                                let index = identity.guard_level as usize % USER_COLORS.len();
                                Color::from_str(USER_COLORS[index]).unwrap()
                            } else {
                                Color::from_str(USER_COLORS[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from(msg.content),
                ])
            }
            MsgType::SuperChat => {
                let msg = serde_json::from_str::<SuperChatMsg>(&b).unwrap();
                let time = get_local_time_from_timestamp(msg.timestamp);

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    {
                        if render_type {
                            Span::from(format!(" [{}] ", t)).fg(Color::Yellow)
                        } else {
                            Span::raw(" ")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.name)).bg(color)
                        } else {
                            Span::raw("")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.level))
                                .fg(color)
                                .bg(Color::White)
                        } else {
                            Span::raw("")
                        }
                    },
                    Span::raw(" "),
                    {
                        let color = {
                            if let Some(identity) = msg.user.identity {
                                let index = identity.guard_level as usize % USER_COLORS.len();
                                Color::from_str(USER_COLORS[index]).unwrap()
                            } else {
                                Color::from_str(USER_COLORS[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from(format!("({} 元)", msg.price)),
                    Span::raw(" "),
                    Span::from(msg.content),
                ])
            }
            MsgType::Gift => {
                let msg = serde_json::from_str::<GiftMsg>(&b).unwrap();
                let time = get_local_time_from_timestamp(msg.timestamp);

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    {
                        if render_type {
                            Span::from(format!(" [{}] ", t)).fg(Color::Yellow)
                        } else {
                            Span::raw(" ")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.name)).bg(color)
                        } else {
                            Span::raw("")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.level))
                                .fg(color)
                                .bg(Color::White)
                        } else {
                            Span::raw("")
                        }
                    },
                    Span::raw(" "),
                    {
                        let color = {
                            if let Some(identity) = msg.user.identity {
                                let index = identity.guard_level as usize % USER_COLORS.len();
                                Color::from_str(USER_COLORS[index]).unwrap()
                            } else {
                                Color::from_str(USER_COLORS[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from("赠送了"),
                    Span::raw(" "),
                    Span::from(msg.gift_name),
                    Span::raw(" "),
                    Span::from(format!("* {}", msg.amount)),
                    Span::raw(" "),
                    {
                        let total = (msg.price * msg.amount) as f32 / 1000.0;

                        if total == 0.0 {
                            Span::raw("")
                        } else {
                            Span::from(format!("({:.1} 元)", total))
                                .fg(Color::LightMagenta)
                                .bold()
                        }
                    },
                    {
                        if let Some(master) = msg.send_master {
                            Span::from(format!(" 给 {}", master.uname))
                        } else {
                            Span::raw("")
                        }
                    },
                ])
            }
            MsgType::GuardBuy => {
                let msg = serde_json::from_str::<GuardBuyMsg>(&b).unwrap();
                let time = get_local_time_from_timestamp(msg.timestamp);

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    {
                        if render_type {
                            Span::from(format!(" [{}] ", t)).fg(Color::Yellow)
                        } else {
                            Span::raw(" ")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.name)).bg(color)
                        } else {
                            Span::raw("")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.level))
                                .fg(color)
                                .bg(Color::White)
                        } else {
                            Span::raw("")
                        }
                    },
                    Span::raw(" "),
                    {
                        let color = {
                            if let Some(identity) = msg.user.identity {
                                let index = identity.guard_level as usize % USER_COLORS.len();
                                Color::from_str(USER_COLORS[index]).unwrap()
                            } else {
                                Color::from_str(USER_COLORS[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from("在你的直播间购买了"),
                    Span::from(msg.gift_name).fg(tailwind::GREEN.c400).bold(),
                    Span::raw(" "),
                    Span::from(format!("({} 元)", msg.price / 1000)),
                ])
            }
            MsgType::UserAction => {
                let msg = serde_json::from_str::<UserActionMsg>(&b).unwrap();
                let time = get_local_time_from_timestamp(msg.timestamp);

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    {
                        if render_type {
                            Span::from(format!(" [{}] ", t)).fg(Color::Yellow)
                        } else {
                            Span::raw(" ")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.name)).bg(color)
                        } else {
                            Span::raw("")
                        }
                    },
                    {
                        if let Some(ref badge) = msg.user.badge {
                            let color = {
                                if let Some(ref anchor) = badge.anchor {
                                    if let Some(is_same_room) = anchor.is_same_room {
                                        if is_same_room {
                                            Color::from_str(&badge.color).unwrap()
                                        } else {
                                            Color::from_str("#666666").unwrap()
                                        }
                                    } else {
                                        Color::from_str("#666666").unwrap()
                                    }
                                } else {
                                    Color::from_str("#666666").unwrap()
                                }
                            };

                            Span::from(format!(" {} ", badge.level))
                                .fg(color)
                                .bg(Color::White)
                        } else {
                            Span::raw("")
                        }
                    },
                    Span::raw(" "),
                    {
                        let color = {
                            if let Some(identity) = msg.user.identity {
                                let index = identity.guard_level as usize % USER_COLORS.len();
                                Color::from_str(USER_COLORS[index]).unwrap()
                            } else {
                                Color::from_str(USER_COLORS[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    {
                        match msg.action.as_str() {
                            "enter" => Span::from("进入"),
                            "follow" => Span::from("关注"),
                            "share" => Span::from("分享"),
                            "like" => Span::from("点赞"),
                            _ => Span::raw(""),
                        }
                    },
                    Span::from("直播间"),
                ])
            }
        }
    }
}

#[derive(Clone)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
    pub state: ListState,
}

impl Tabs {
    pub fn next_tab(&mut self) {
        let selected = self.state.selected().unwrap();
        if selected + 1 < self.tabs.len() {
            self.state.select(Some(selected + 1));
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn previous_tab(&mut self) {
        let selected = self.state.selected().unwrap();
        if selected > 0 {
            self.state.select(Some(selected - 1));
        } else {
            self.state.select(Some(self.tabs.len() - 1));
        }
    }
}

impl fmt::Debug for Tabs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tabs").finish()
    }
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

fn get_local_time_from_timestamp(timestamp: i64) -> DateTime<Local> {
    let time = DateTime::from_timestamp_millis(timestamp).unwrap();
    let time = time.naive_local();
    Local.from_utc_datetime(&time)
}
