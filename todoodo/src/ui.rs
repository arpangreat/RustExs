use std::collections::HashMap;

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

pub struct App {
    pub key_input: String,
    pub value_input: bool,
    pub pairs: HashMap<String, bool>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: false,
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = false;
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_pairs(&self) -> std::io::Result<()> {
        let output = &self.pairs;
        println!("{:?}", output);
        Ok(())
    }

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
