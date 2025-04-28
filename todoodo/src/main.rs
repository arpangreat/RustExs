use std::io;
use std::io::Write;

use crossterm::execute;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::terminal::{self, enable_raw_mode};
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use rusqlite::types::Value;
use todoodo::todo::Todo;
use todoodo::ui::{App, CurrentScreen, CurrentlyEditing};

fn main() -> std::io::Result<(), Box<dyn Error>> {
    let mut todoapp = Todo::new()?;

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Invalid run");
    }

    let action = &args[1];

    match action.as_str() {
        "add" => {
            print!("Add a description for the Todo: ");
            std::io::stdout().flush().unwrap();

            let mut desc = String::new();

            std::io::stdin()
                .read_line(&mut desc)
                .expect("Please write down your Todo description");

            let desc = desc.trim().parse().expect("yo");

            todoapp.add_task(desc)?;
        }
        "check" => {
            print!("Which task do you wanna checked? ");
            std::io::stdout().flush().unwrap();

            let mut num = String::new();

            std::io::stdin()
                .read_line(&mut num)
                .expect("Please type a valid number");

            let num: usize = num.trim().parse().expect("Please type a number");

            todoapp.check_task(num)?;
        }
        "list" => {
            println!("Todo: ");
            todoapp.list()?;
        }
        _ => println!("Tell us a valid command. Available actions are add, check, list"),
    }

    enable_raw_mode()?;
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend);

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        EnableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_pairs()?;
        } else if let Err(err) = res {
            println!("{err:?}");
        }
    };

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut terminal<B>, app: &mut App) -> std::io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read() {
            dbg!(key.code)
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },

                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    app.save_key_value();
                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                }
                                CurrentlyEditing::Value => app.value_input.pop(),
                            }
                        }
                    }

                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None
                    }

                    KeyCode::Tab => {
                        app.toggle_editing();
                    }

                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(value);
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.push(value);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}
