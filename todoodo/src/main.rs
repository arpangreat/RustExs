#![feature(mixed_integer_ops_unsigned_sub)]
use color_eyre::Result;
use todoodo::{app::App, todo::Todo};

fn main() -> Result<()> {
    // let mut todoapp = Todo::new()?;

    // let args: Vec<String> = std::env::args().collect();

    // if args.len() < 2 {
    //     println!("Invalid run");
    // }

    // let action = &args[1];

    // match action.as_str() {
    //     "add" => {
    //         print!("Add a description for the Todo: ");
    //         // std::io::stdout().flush().unwrap();

    //         let mut desc = String::new();

    //         std::io::stdin()
    //             .read_line(&mut desc)
    //             .expect("Please write down your Todo description");

    //         let desc = desc.trim().parse().expect("yo");

    //         {
    //             let this = &mut todoapp;
    //             this.conn
    //                 .execute(
    //                     "
    //         INSERT INTO Todo (description) VALUES (?1)
    //         ",
    //                     [desc.key_input.clone()],
    //                 )
    //                 .expect("Error on adding the task in database");

    //             println!("task added");

    //             Ok(())
    //         }?;
    //     }
    //     "check" => {
    //         print!("Which task do you wanna checked? ");
    //         // std::io::stdout().flush().unwrap();

    //         let mut num = String::new();

    //         std::io::stdin()
    //             .read_line(&mut num)
    //             .expect("Please type a valid number");

    //         let num: usize = num.trim().parse().expect("Please type a number");

    //         todoapp.check_task(num)?;
    //     }
    //     "list" => {
    //         println!("Todo: ");
    //         todoapp.list()?;
    //     }
    //     _ => println!("Tell us a valid command. Available actions are add, check, list"),
    // }

    color_eyre::install()?;

    let mut todo = Todo::new()?;
    todo.list()?;

    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
