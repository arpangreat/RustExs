use std::error::Error;
use std::io;
use std::io::Write;

use crossterm::execute;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, EnableMouseCapture, Event, KeyCode};
use rusqlite::types::Value;
use todoodo::app::{App, CurrentScreen};
use todoodo::todo::Todo;

use color_eyre;

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

    color_eyre::install()?;

    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
