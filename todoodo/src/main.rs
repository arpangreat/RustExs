struct Task {
    description: String,
    completed: bool,
}

struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    fn new() -> Self {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            description,
            completed: false,
        };

        self.tasks.push(task);
    }

    fn check_task(&mut self, task: usize) {
        self.tasks.iter_mut().enumerate().for_each(|(i, tasks)| {
            if i == task {
                tasks.completed = true
            }
        });
    }

    fn list(&mut self) {
        for (i, tasks) in self.tasks.iter().enumerate() {
            // add code here
            println!(
                "{} [{}] {}",
                i,
                if tasks.completed { 'x' } else { ' ' },
                tasks.description
            )
        }
    }
}

fn main() {
    let mut todoapp = Todo::new();

    todoapp.add_task("create a project".to_string());
    todoapp.add_task("create a repo".to_string());

    todoapp.check_task(1);

    println!("Todo: ");
    todoapp.list();
}
