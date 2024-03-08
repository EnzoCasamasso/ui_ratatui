use ratatui::{prelude::*, widgets::*};

pub struct InputContent;

impl Widget for InputContent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::raw("Custom widget");
    }
}
