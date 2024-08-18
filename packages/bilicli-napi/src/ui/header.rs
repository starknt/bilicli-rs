use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};

use crate::api::RoomInfo;

#[derive(Clone)]
pub struct Header {
    info: RoomInfo,
}

impl Header {
    pub fn new(info: RoomInfo) -> Self {
        Self { info }
    }
}

impl Widget for &Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .padding(Padding::horizontal(1));

        Paragraph::new(
            Line::from(vec![
                Span::from(format!("{}", self.info.room_id)).fg(Color::Green),
                Span::raw(" "),
                Span::from(format!(
                    "({}·{})",
                    self.info.area_name, self.info.parent_area_name
                ))
                .fg(Color::LightGreen),
                Span::raw(" "),
                Span::from(self.info.title.clone()),
            ])
            .bold(),
        )
        .block(block)
        .render(area, buf);
    }
}
