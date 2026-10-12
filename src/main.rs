use std::fs::OpenOptions;
use std::io::Write;

enum Command {
    Add(String),
    List,
    Done(usize),
}
fn main() {
    let args = std::env::args().skip(1).collect();

    match parse_command(args) {
        Ok(r) => execute(r),
        Err(error) => println!("{}", error),
    }
}

fn show_task(index: usize, task: &str) {
    println!("{}. {}", index + 1, task);
}

fn add_task(task: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tasks.txt")?;

    writeln!(file, "{task}")?;
    Ok(())
}

fn execute(command: Command) {
    match command {
        Command::Add(text) => match add_task(&text) {
            Ok(()) => println!("Added: {text}"),
            Err(err) => println!("{}", err),
        },
        Command::List => {
            let file_read = match std::fs::read_to_string("tasks.txt") {
                Ok(f) => f,
                Err(error) => {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        println!("No tasks yet");
                        return;
                    }
                    println!("Error while trying to read file, {error}");
                    return;
                }
            };

            if file_read.is_empty() {
                println!("No tasks yet");
                return;
            }

            for (index, line) in file_read.lines().enumerate() {
                show_task(index, line);
            }
        }
        Command::Done(task_number) => {
            let mut tasks: Vec<String> = Vec::new();
            match std::fs::read_to_string("tasks.txt") {
                Ok(f) => {
                    if f.is_empty() {
                        println!("No tasks yet");
                        return;
                    } else {
                        for line in f.lines() {
                            tasks.push(String::from(line));
                        }
                    }
                }
                Err(error) => {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        println!("No tasks yet");
                    } else {
                        println!("Error while trying to read file, {error}");
                    }
                    return;
                }
            };
            if 1 <= task_number && task_number <= tasks.len() {
                println!("Selected Task {}", task_number);
                tasks.remove(task_number - 1);
                let mut contents = tasks.join("\n");
                if !tasks.is_empty() {
                    contents.push_str("\n");
                }
                match std::fs::write("tasks.txt", contents) {
                    Ok(()) => println!("Task removed successfully"),
                    Err(err) => println!("Error while trying to remove a task: {}", err),
                }
            } else {
                println!("Task number out of range");
            }
        }
    };
}
fn parse_command(args: Vec<String>) -> Result<Command, String> {
    match args.first() {
        None => Err(String::from("Missing Command")),
        Some(command_name) => {
            if command_name == "list" {
                Ok(Command::List)
            } else if command_name == "add" {
                match args.get(1) {
                    None => Err(String::from("Missing task text")),
                    Some(task) => Ok(Command::Add(task.clone())),
                }
            } else if command_name == "done" {
                match args.get(1) {
                    None => Err(String::from("Missing task number")),
                    Some(task_text) => match task_text.parse::<usize>() {
                        Ok(task_number) => {
                            if 1 <= task_number {
                                Ok(Command::Done(task_number))
                            } else {
                                Err(String::from("Task number out of bounds"))
                            }
                        }
                        Err(error) => {
                            Err(format!("Error while trying to parse task number: {error}"))
                        }
                    },
                }
            } else {
                Err(String::from("Unknown Command Error"))
            }
        }
    }
}
