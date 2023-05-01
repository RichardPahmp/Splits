use ratatui::style::{Color, Style};

pub fn red() -> Style {
    Style::default().fg(Color::Red)
}

pub fn green() -> Style {
    Style::default().fg(Color::Green)
}

pub fn yellow() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn white() -> Style {
    Style::default().fg(Color::White)
}
