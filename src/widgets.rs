
use ratatui::{prelude::*, widgets::*};

pub struct Button {
    label: String,
    is_pressed: bool,
    style: Style,
    pressed_style: Option<Style>,
}

impl Widget for Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.is_pressed {
            self.pressed_style.unwrap_or_else(|| Style::default().fg(Color::Blue))
        } else {
            self.style
        };
        buf.set_string(area.left(), area.top(), &self.label, style);
    } 
}

//There is a StatefulWidget witch 'remember' a information between two draw calls

