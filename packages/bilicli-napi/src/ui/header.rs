use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};

use crate::api::RoomInfo;

#[derive(Clone, Default)]
pub struct Header {
    info: Option<RoomInfo>,
}

impl Header {
    pub fn update_info(&mut self, info: RoomInfo) {
        self.info = Some(info);
    }
}

impl Widget for &Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1));

        if let Some(info) = &self.info {
            Paragraph::new(
                Line::from(vec![
                    Span::from(format!("{}", info.room_id)).fg(Color::Green),
                    Span::raw(" "),
                    Span::from(format!("({}·{})", info.area_name, info.parent_area_name))
                        .fg(Color::LightGreen),
                    Span::raw(" "),
                    Span::from(info.title.clone()),
                ])
                .bold(),
            )
            .block(block)
            .render(area, buf);
        } else {
            Paragraph::new(
                Line::from(vec![
                    Span::from("正在获取房间信息...").fg(Color::Green),
                    Span::raw(" "),
                    Span::from(format!("({}·{})", "", "")).fg(Color::LightGreen),
                    Span::raw(" "),
                    Span::from(""),
                ])
                .bold(),
            )
            .block(block)
            .render(area, buf);
        }
    }
}
