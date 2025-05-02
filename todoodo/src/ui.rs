use std::vec;

use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize, palette::material::RED},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::app::{App, CurrentScreen};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    let (msg, style) = match app.current_screen {
        CurrentScreen::Normal => (
            vec![
                "Press: ".into(),
                "e".bold(),
                " to add or delete a Todo ".into(),
                "q".bold(),
                " to quit ".into(),
            ],
            Style::default().fg(Color::Red),
        ),
        CurrentScreen::Editing => (
            vec![
                "Press: ".into(),
                "Enter".bold(),
                " To confirm ".into(),
                "Esc".bold(),
                " To go back to normal mode ".into(),
            ],
            Style::default().fg(Color::Red),
        ),
    };

    let text = Text::from(Line::from(msg)).patch_style(style);
    f.render_widget(text, chunks[0]);

    let input = Paragraph::new(app.key_input.as_str()).style(match app.current_screen {
        CurrentScreen::Editing => Style::default(),
        CurrentScreen::Normal => Style::default().add_modifier(Modifier::DIM),
    });
    f.render_widget(input, chunks[1]);
}
