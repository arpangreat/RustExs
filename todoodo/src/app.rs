// Crossterm backend imports
use crossterm::event::{self, Event, KeyCode};

// Ratatui imports
use ratatui::DefaultTerminal;

use crate::todo::Todo;

use crate::ui::ui;

pub struct App {
    pub key_input: String,
    pub value_input: bool,
    pub todos: Todo,
    pub current_screen: CurrentScreen,
}

pub enum CurrentScreen {
    Normal,
    Editing,
}

impl App {
    pub fn new() -> App {
        let todos = Todo::new().expect("Todo initialization failed");
        App {
            key_input: String::new(),
            value_input: false,
            current_screen: CurrentScreen::Normal,
            todos,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, &self))?;
            if let Event::Key(key) = event::read()? {
                match self.current_screen {
                    CurrentScreen::Normal => match key.code {
                        KeyCode::Char('e') => self.current_screen = CurrentScreen::Editing,
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('j') => {
                            if !self.todos.tasks.is_empty() {
                                self.todos.selected_tasks = Some(match self.todos.selected_tasks {
                                    Some(i) => {
                                        if i >= self.todos.tasks.len() {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                });
                            }
                        }

                        KeyCode::Char('k') => {
                            if !self.todos.tasks.is_empty() {
                                self.todos.selected_tasks = Some(match self.todos.selected_tasks {
                                    Some(i) => {
                                        if i == 0 {
                                            self.todos.tasks.len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                });
                            }
                        }

                        _ => {}
                    },

                    CurrentScreen::Editing => match key.code {
                        KeyCode::Enter => {
                            self.todos.add_task(&self.key_input)?;
                            self.current_screen = CurrentScreen::Normal
                        }
                        KeyCode::Char(c) => {
                            self.key_input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.key_input.pop();
                        }

                        KeyCode::Esc => {
                            self.current_screen = CurrentScreen::Normal;
                        }

                        _ => {}
                    },
                }
            }
        }

        Ok(())
    }
}
