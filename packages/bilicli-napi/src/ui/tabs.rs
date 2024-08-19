use std::{fmt, str::FromStr};

use chrono::{Local, NaiveDateTime, TimeZone};
use ratatui::{
    prelude::*,
    widgets::{Block, ListState, Padding, Paragraph, Scrollbar, ScrollbarState},
};

use crate::CliState;

use super::{DanmuMsg, GiftMsg, GuardBuyMsg, MsgType, SuperChatMsg, UserActionMsg};

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
    fn render_all_tab(&mut self, area: Rect, buf: &mut Buffer, state: &mut CliState) {
        let text: Vec<Line<'static>> = state
            .messages
            .iter()
            .map(|(t, b)| Self::render_msg(*t, b.clone()))
            .collect();

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
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
            .map(|(t, b)| Self::render_msg(*t, b.clone()))
            .collect();

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
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
            .map(|(t, b)| Self::render_msg(*t, b.clone()))
            .collect();

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
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
            .map(|(t, b)| Self::render_msg(*t, b.clone()))
            .collect();

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
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
            .map(|(t, b)| Self::render_msg(*t, b.clone()))
            .collect();

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
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
            .map(|(t, b)| Self::render_msg(*t, b.clone()))
            .collect();

        self.set_state_content_length(text.len());

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .scroll((self.scroll(), 0))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("w"))
            .end_symbol(Some("s"));

        StatefulWidget::render(scrollbar, area, buf, self.state());
    }
}

impl Tab {
    fn render_msg(t: MsgType, b: String) -> Line<'static> {
        let user_colors = vec!["#967E76", "#FF7C28", "#E17AFF", "#00D1F1"];

        match t {
            MsgType::Danmu => {
                let msg = serde_json::from_str::<DanmuMsg>(&b).unwrap();
                let time = NaiveDateTime::from_timestamp(msg.timestamp, 0);
                let time = Local.from_local_datetime(&time).unwrap();

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    Span::raw(" "),
                    Span::from(format!("[{}]", t)).fg(Color::Yellow),
                    Span::raw(" "),
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
                                let index = identity.guard_level as usize % user_colors.len();
                                Color::from_str(user_colors[index]).unwrap()
                            } else {
                                Color::from_str(user_colors[0]).unwrap()
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
                let time = NaiveDateTime::from_timestamp(msg.timestamp, 0);
                let time = Local.from_local_datetime(&time).unwrap();

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    Span::raw(" "),
                    Span::from(format!("[{}]", t)).fg(Color::Yellow),
                    Span::raw(" "),
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
                                let index = identity.guard_level as usize % user_colors.len();
                                Color::from_str(user_colors[index]).unwrap()
                            } else {
                                Color::from_str(user_colors[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from(format!("(¥ {})", msg.price)),
                    Span::raw(" "),
                    Span::from(msg.content),
                ])
            }
            MsgType::Gift => {
                let msg = serde_json::from_str::<GiftMsg>(&b).unwrap();
                let time = NaiveDateTime::from_timestamp(msg.timestamp, 0);
                let time = Local.from_local_datetime(&time).unwrap();

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    Span::raw(" "),
                    Span::from(format!("[{}]", t)).fg(Color::Yellow),
                    Span::raw(" "),
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
                                let index = identity.guard_level as usize % user_colors.len();
                                Color::from_str(user_colors[index]).unwrap()
                            } else {
                                Color::from_str(user_colors[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from("赠送了"),
                    Span::from(format!("(¥ {})", msg.price / 1000)),
                    Span::raw(" "),
                    Span::from(msg.gift_name),
                    Span::raw(" "),
                    Span::from(format!(" * {}", msg.amount)),
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
                let time = NaiveDateTime::from_timestamp(msg.timestamp, 0);
                let time = Local.from_local_datetime(&time).unwrap();

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    Span::raw(" "),
                    Span::from(format!("[{}]", t)).fg(Color::Yellow),
                    Span::raw(" "),
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
                                let index = identity.guard_level as usize % user_colors.len();
                                Color::from_str(user_colors[index]).unwrap()
                            } else {
                                Color::from_str(user_colors[0]).unwrap()
                            }
                        };

                        Span::from(msg.user.uname).bold().fg(color)
                    },
                    Span::raw(": "),
                    Span::from(format!("(¥ {})", msg.price / 1000)),
                    Span::raw(" "),
                    Span::from(msg.gift_name),
                ])
            }
            MsgType::UserAction => {
                let msg = serde_json::from_str::<UserActionMsg>(&b).unwrap();
                let time = NaiveDateTime::from_timestamp(msg.timestamp, 0);
                let time = Local.from_local_datetime(&time).unwrap();

                Line::from(vec![
                    Span::from(format!("{}", time.format("%H:%M:%S")))
                        .fg(Color::from_hsl(0.0, 0.0, 40.0)),
                    Span::raw(" "),
                    Span::from(format!("[{}]", t)).fg(Color::Yellow),
                    Span::raw(" "),
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
                                let index = identity.guard_level as usize % user_colors.len();
                                Color::from_str(user_colors[index]).unwrap()
                            } else {
                                Color::from_str(user_colors[0]).unwrap()
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
