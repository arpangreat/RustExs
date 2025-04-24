use rusqlite::{Connection, Result};
use std::io::Write;

struct Todo {
    conn: Connection,
}

impl Todo {
    fn new() -> Result<Self> {
        let conn = Connection::open("task.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Todo (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE
                )",
            [],
        )?;

        Ok(Todo { conn })
    }

    fn add_task(&mut self, description: String) -> Result<()> {
        self.conn.execute(
            "
            INSERT INTO Todo (description) VALUES (?1)
            ",
            [description],
        )?;

        println!("task added");

        Ok(())
    }

    fn check_task(&mut self, id: usize) -> Result<()> {
        let affected = self.conn.execute(
            "
            UPDATE Todo SET completed = 1 WHERE id = ?1
            ",
            [id],
        )?;

        if affected == 0 {
            println!("No task of id: {} found", id);
        } else {
            println!("Task of id: {} completed", id);
        };

        Ok(())
    }

    fn list(&mut self) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, completed FROM Todo")?;

        let tasks = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
            ))
        })?;

        for task in tasks {
            let (id, description, completed) = task?;

            println!("Todo: ");
            println!("id: {:?}", id);
            println!("description: {:?}", description);
            println!("completed: {:?}", completed);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
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

    Ok(())
}
