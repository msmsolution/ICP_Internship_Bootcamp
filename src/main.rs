extern crate uuid;

#[derive(Debug, Clone)]

//Create a struct called Task
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

//Create a vector to store instances of the Task struct
struct TodoList {
    task_list: Vec<Task>,
}
//Implement the add_task function
impl Task {
    fn add_task(description: &str) -> Task {
        
        let task = Task {
            id: usize::from_str_radix(&uuid::Uuid::new_v4().to_string()[..8], 16).unwrap(),
            description: description.to_string(),
            completed: false,
        };
        let mut todo_list = TodoList {
            task_list: Vec::new(),
        };
        todo_list.task_list.push(task.clone());
        task
    }
}
//Implement the complete_task function
impl TodoList {
    fn complete_task(&self, id: usize) -> Option<&Task> {
        if let Some(task) = self.task_list.iter().find(|t| t.id == id) {
            Some(task)
        } else {
            None
        }
    }
}
//Implement the list_tasks function
impl TodoList {
    fn list_tasks(&self) {
        for task in &self.task_list {
            println!(
                "Unique_ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }    
}

fn main() {
    println!("ADDED TASKS:");
    println!("--------------------------------------------------------");
    //Print out the tasks
    let task1 = Task::add_task("Do the dishes");
    let task2 = Task::add_task("Take out the trash");
    let task3 = Task::add_task("Finish my Rust Programming Homework");
    
    println!("{:?} {:?} {:?}", task1, task2, task3);

    println!("--------------------------------------------------------");
    println!("LIST ALL TASKS:");
    println!("--------------------------------------------------------");

    let todo_list = TodoList {
        task_list: vec![task1, task2, task3],
    };
    todo_list.list_tasks();
    println!("--------------------------------------------------------");
}
