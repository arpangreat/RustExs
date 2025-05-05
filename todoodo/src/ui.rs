use std::vec;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &mut App) {
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
                " to add or delete a Todo, ".into(),
                "q".bold(),
                " to quit, ".into(),
                "j/k".bold(),
                " to scroll and select todo ".into(),
            ],
            Style::default().fg(Color::Red),
        ),
        CurrentScreen::Editing => (
            vec![
                "Press: ".into(),
                "Enter".bold(),
                " To confirm, ".into(),
                "Esc".bold(),
                " To go back to normal mode ".into(),
            ],
            Style::default().fg(Color::Red),
        ),
    };

    let text = Text::from(Line::from(msg)).patch_style(style);
    f.render_widget(text, chunks[0]);

    let input = Paragraph::new(app.key_input.as_str())
        .style(match app.current_screen {
            CurrentScreen::Editing => Style::default(),
            CurrentScreen::Normal => Style::default().add_modifier(Modifier::DIM),
        })
        .block(Block::default().borders(Borders::all()).title("New task"));
    f.render_widget(input, chunks[1]);

    match app.current_screen {
        CurrentScreen::Normal => {}
        CurrentScreen::Editing => {
            f.set_cursor(
                chunks[1].x + app.key_input.len() as u16 + 1,
                chunks[1].y + 1,
            );
        }
    }

    // if let task_status = if app.todos.tasks.compl

    let items: Vec<ListItem> = app
        .todos
        .tasks
        .iter()
        .map(|task| {
            let status: Span = if task.completed {
                Span::styled(format!("[✅]"), Style::default().fg(Color::Green).bold())
            } else {
                Span::styled(format!("[❌]"), Style::default().fg(Color::Red).bold())
            };
            ListItem::new(Line::from(vec![
                status,
                Span::styled(format!(" {}. ", task.id), Style::default()),
                Span::styled(
                    format!("{}", task.description),
                    Style::default().fg(Color::Green).italic(),
                ),
            ]))
            .style(Style::default().fg(Color::White))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Tasks").borders(Borders::all()))
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_stateful_widget(list, chunks[2], &mut app.list_state);

    let status = format!(
        "Total: {} | Completed: {}",
        app.todos.tasks.len(),
        app.todos.tasks.iter().filter(|t| t.completed).count()
    );
    let status_bar = Paragraph::new(status).style(Style::default().fg(Color::White));
    f.render_widget(status_bar, chunks[3]);
}
