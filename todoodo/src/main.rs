use std::io;
use std::io::Write;

use todoodo::todo::Todo;
use todoodo::ui::App;

fn main() -> std::io::Result<()> {
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

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();

    app_result
}
