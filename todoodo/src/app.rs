// Crossterm backend imports
use crossterm::event::{self, Event, KeyCode};

use ratatui::widgets::ListState;
// Ratatui imports
use ratatui::DefaultTerminal;

use crate::todo::Todo;

use crate::ui::ui;

pub struct App {
    pub key_input: String,
    pub value_input: bool,
    pub todos: Todo,
    pub current_screen: CurrentScreen,
    pub list_state: ListState,
}

pub enum CurrentScreen {
    Normal,
    Editing,
}

impl App {
    pub fn new() -> App {
        let mut todos = Todo::new().expect("Todo initialization failed");
        todos.refresh_task().expect("Refresh failed");

        App {
            key_input: String::new(),
            value_input: false,
            current_screen: CurrentScreen::Normal,
            todos,
            list_state: {
                let mut s = ListState::default();
                s.select(Some(0));
                s
            },
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, self))?;
            if let Event::Key(key) = event::read()? {
                match self.current_screen {
                    CurrentScreen::Normal => match key.code {
                        KeyCode::Char('e') => self.current_screen = CurrentScreen::Editing,
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('j') => {
                            if let Some(selected) = self.list_state.selected() {
                                let next = if selected + 1 >= self.todos.tasks.len() {
                                    0 // wrap around to top
                                } else {
                                    selected + 1
                                };
                                self.list_state.select(Some(next));
                            } else if !self.todos.tasks.is_empty() {
                                self.list_state.select(Some(0));
                            }
                        }

                        KeyCode::Char('k') => {
                            if let Some(selected) = self.list_state.selected() {
                                let prev = if selected == 0 {
                                    self.todos.tasks.len().saturating_sub(1) // wrap around to bottom
                                } else {
                                    selected - 1
                                };
                                self.list_state.select(Some(prev));
                            } else if !self.todos.tasks.is_empty() {
                                self.list_state.select(Some(0));
                            }
                        }

                        KeyCode::Char(' ') => {
                            if let Some(selected) = self.list_state.selected() {
                                let next = selected.saturating_sub(1);
                                self.todos.selected_tasks = Some(next);

                                if let Some(task) = self.todos.tasks.get(selected) {
                                    self.todos.check_task(Some(task.id.into()))?;
                                }
                            } else {
                                self.todos.selected_tasks = Some(0)
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
