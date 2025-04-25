// Crossterm backend imports
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

// Ratatui imports
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Table, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame));
            self.handle_events();
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('k') => self.up(),
            KeyCode::Char('j') => self.down(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn up(&mut self) {
        self.counter -= 1;
    }

    fn down(&mut self) {
        self.counter += 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Todo-Doo".bold());
        let instruction = Line::from(vec![
            " Input ".into(),
            "<Up>".yellow().bold(),
            " K ".into(),
            "<Down>".yellow().bold(),
            " J ".into(),
            "<Quit>".red().bold(),
            " q ".into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instruction.centered())
            .border_set(border::EMPTY);

        let TodoText = Text::from(vec![Line::from(vec![
            "Todo".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(TodoText)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
