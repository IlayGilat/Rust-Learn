use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut args = std::env::args().skip(1);

    let first_arg = args.next().unwrap_or_default();

    if first_arg == "" {
        println!("No arguments provided");
        return;
    }

    if first_arg == "add" {
        let second_arg = args.next();
        if second_arg == None {
            println!("Missing Text");
            return;
        }

        let print_second_arg = second_arg.unwrap();

        match add_task(&print_second_arg) {
            Ok(()) => println!("Added: {print_second_arg}"),
            Err(err) => println!("{}", err),
        }
    } else if first_arg == "list" {
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
    } else if first_arg == "done" {
        let second_arg = args.next();
        if second_arg == None {
            println!("Missing task number");
            return;
        }

        let number_text = second_arg.unwrap();

        let task_number = match number_text.parse::<usize>() {
            Ok(r) => r,
            Err(err) => {
                println!("Error while getting index arg: {}", err);
                return;
            }
        };
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
    } else {
        println!("Unknown Command")
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
