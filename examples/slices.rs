fn main() {
    let mut tasks: Vec<String> = Vec::new();
    tasks.push(String::from("Task 1"));
    tasks.push(String::from("Task 2"));
    append_task(&mut tasks, "Task 3");
    add_suffix_to_all(&mut tasks);
    for task in &tasks {
        println!("{}", task);
    }
}

fn add_suffix_to_all(tasks: &mut [String]) {
    for task in tasks {
        task.push_str(" today");
    }
}

fn append_task(tasks: &mut Vec<String>, text: &str) {
    let task = String::from(text);
    tasks.push(task);
}
