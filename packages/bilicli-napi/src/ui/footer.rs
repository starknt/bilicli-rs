use chrono::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};

use crate::CliState;

#[derive(Clone, Debug, Default)]
pub struct Footer;

// impl FooterState {
//     pub fn new(attention: u32, watchers: String, is_live: bool, start_time: NaiveDateTime) -> Self {
//         Self {
//             attention,
//             watchers,
//             is_live,
//             start_time,
//         }
//     }

//     pub fn update_info(&mut self, info: RoomInfo) {
//         self.attention = info.attention;
//         self.is_live = info.live_status == 1;
//         self.start_time =
//             NaiveDateTime::parse_from_str(&info.live_time, "%Y-%m-%d %H:%M:%S").unwrap_or_default();
//     }

//     pub fn update_attention(&mut self, attention: u32) {
//         self.attention = attention;
//     }

//     pub fn update_watcher(&mut self, watcher: String) {
//         self.watchers = watcher;
//     }

//     pub fn update_live(&mut self, live: bool) {
//         self.is_live = live;
//     }
// }

impl StatefulWidget for &mut Footer {
    type State = CliState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [left, right] = Layout::horizontal([Constraint::Percentage(40), Constraint::Fill(1)])
            .flex(layout::Flex::SpaceBetween)
            .areas(area);

        let [info_area, watcher_area, attention_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(16),
            Constraint::Length(16),
        ])
        .flex(layout::Flex::SpaceBetween)
        .areas(right);

        Paragraph::new(Line::from("按 Enter 输入弹幕信息, Esc 取消输入").fg(Color::LightGreen))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1))
                    .title("提示")
                    .title_style(Style::default().fg(Color::Red))
                    .title_alignment(Alignment::Center)
                    .style(Style::default().red()),
            )
            .render(left, buf);

        if state.is_live {
            let text = format_duration(state.start_time);
            Paragraph::new(Line::from(vec![
                "🔴"
                    .to_string()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
                Span::from("  "),
                Span::from(text),
                Span::from("  "),
                Span::from(format!("(Start at {})", state.start_time.format("%H:%M"))),
            ]))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .render(info_area, buf);
        } else {
            Paragraph::new(Line::from(vec![
                "⚫️"
                    .to_string()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
                Span::from("  "),
                Span::from("未开播").fg(Color::Red),
            ]))
            .block(
                Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .padding(Padding::horizontal(1)),
            )
            .render(info_area, buf);
        }

        Paragraph::new(Line::from(
            format!("👀 {}", state.watchers)
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .padding(Padding::horizontal(1)),
        )
        .render(watcher_area, buf);

        Paragraph::new(Line::from(
            format!("🔥 {}", state.attention)
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .padding(Padding::horizontal(1)),
        )
        .render(attention_area, buf);
    }
}

fn format_duration(start_time: NaiveDateTime) -> String {
    let now = Local::now().naive_local();
    let diff = now.signed_duration_since(start_time);
    let seconds = diff.num_seconds();

    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let secs = seconds - hours * 3600 - minutes * 60;

    let format_hours = {
        if hours > 0 {
            format!("{}", hours).to_string()
        } else {
            "".to_string()
        }
    };

    let format_minutes = {
        if hours > 0 {
            if minutes < 10 {
                format!("0{}", minutes).to_string()
            } else {
                format!("{}", minutes).to_string()
            }
        } else {
            format!("{}", minutes).to_string()
        }
    };

    let format_seconds = {
        if secs < 10 {
            format!("0{}", secs).to_string()
        } else {
            format!("{}", secs).to_string()
        }
    };

    if format_hours.is_empty() {
        format!("{}:{}", format_minutes, format_seconds).to_string()
    } else {
        format!("{}:{}:{}", format_hours, format_minutes, format_seconds).to_string()
    }
}
