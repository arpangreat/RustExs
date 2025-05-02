use rusqlite::Connection;

use crate::app::App;

pub struct Todo {
    pub conn: Connection,
    pub tasks: Vec<TodoItems>,
    pub selected_tasks: Option<usize>,
}

pub struct TodoItems {
    id: u16,
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new() -> std::io::Result<Self> {
        let conn =
            Connection::open("task.db").expect("Error while making a connection to the database");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS Todo (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT FALSE
                )",
            [],
        )
        .expect("Error while evaluating the query");

        Ok(Todo {
            conn,
            tasks: Vec::new(),
            selected_tasks: None,
        })
    }

    pub fn add_task(&mut self, app: &App) -> std::io::Result<()> {
        self.conn
            .execute(
                "
            INSERT INTO Todo (description) VALUES (?1)
            ",
                [app.key_input.clone()],
            )
            .expect("Error on adding the task in database");

        println!("task added");

        Ok(())
    }

    pub fn check_task(&mut self, id: usize) -> std::io::Result<()> {
        let affected = self
            .conn
            .execute(
                "
            UPDATE Todo SET completed = 1 WHERE id = ?1
            ",
                [id],
            )
            .expect("Error when switching the task to completed");

        if affected == 0 {
            println!("No task of id: {} found", id);
        } else {
            println!("Task of id: {} completed", id);
        };

        Ok(())
    }

    pub fn list(&mut self) -> std::io::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, completed FROM Todo")
            .expect("Error when evaluating the query");

        let tasks = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0).expect("Error when getting id"),
                    row.get::<_, String>(1)
                        .expect("Error when getting description"),
                    row.get::<_, bool>(2)
                        .expect("Error when getting completed status"),
                ))
            })
            .expect("Error while getting the Todos");

        for task in tasks {
            let (id, description, completed) =
                task.expect("Error while getting the value of Todos");

            println!("Todo: ");
            println!("id: {:?}", id);
            println!("description: {:?}", description);
            println!("completed: {:?}", completed);
        }

        Ok(())
    }
}
