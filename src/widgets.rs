use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub struct InputContent {
    value: String,
}

impl Widget for InputContent {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
