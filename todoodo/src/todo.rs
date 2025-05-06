use rusqlite::Connection;

pub struct Todo {
    pub conn: Connection,
    pub tasks: Vec<TodoItems>,
    pub selected_tasks: Option<usize>,
}

pub struct TodoItems {
    pub id: u16,
    pub description: String,
    pub completed: bool,
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

    pub fn refresh_task(&mut self) -> std::io::Result<()> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, completed FROM Todo")
            .expect("Error when evaluating the query");

        let tasks = stmt
            .query_map([], |row| {
                Ok(TodoItems {
                    id: row.get::<_, i32>(0).expect("Error when getting id") as u16,
                    description: row
                        .get::<_, String>(1)
                        .expect("Error when getting description"),
                    completed: row
                        .get::<_, bool>(2)
                        .expect("Error when getting completed status"),
                })
            })
            .expect("Error while getting the Todos");

        self.tasks.clear();

        for task in tasks {
            self.tasks
                .push(task.expect("Pushing tasks in TodoItems failed"));
        }
        Ok(())
    }

    pub fn add_task(&mut self, key: &str) -> std::io::Result<()> {
        self.conn
            .execute(
                "
            INSERT INTO Todo (description) VALUES (?1)
            ",
                [key],
            )
            .expect("Error on adding the task in database");

        println!("task added");

        self.refresh_task()?;

        Ok(())
    }

    pub fn check_task(&mut self, id: Option<usize>) -> std::io::Result<()> {
        let id = match id {
            Some(val) => val,
            None => {
                println!("No task seleced");
                return Ok(());
            }
        };

        let completed = {
            let mut stmt = self
                .conn
                .prepare("SELECT completed FROM Todo WHERE id=?1")
                .expect("Error when evaluating the query");

            let mut rows = stmt
                .query([id])
                .expect("Querying for completed status failed");

            if let Some(row) = rows.next().expect("Selecting completed failed") {
                row.get::<_, bool>(0).unwrap()
            } else {
                println!("No task found with id {}", id);
                return Ok(()); // early return — no such task
            }
        };

        let new_status = if completed { 0 } else { 1 };
        self.conn
            .execute(
                "
            UPDATE Todo SET completed = ?1 WHERE id = ?2
            ",
                [new_status, id],
            )
            .expect("Error when switching the task to completed");

        self.refresh_task()?;

        Ok(())
    }

    pub fn delete_task(&mut self, id: usize, verbose: bool) -> std::io::Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM TODO WHERE id = ?1", [id])
            .expect("Deleting task from todo failed");

        if verbose {
            if affected == 0 {
                println!("Deleted task of id: {}", id);
            } else {
                println!("No task found");
            }
        }

        self.refresh_task()?;

        Ok(())
    }
}
