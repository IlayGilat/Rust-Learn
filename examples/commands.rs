
enum Command {
    Add(String),
    List,
    Done(usize),
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse_command(args) {
        Ok(r)=> execute(r),
        Err(error)=> println!("{}",error)
    };
    
}

fn execute(command: Command) {
    match command {
        Command::Add(text) => println!("{}",text),
        Command::List => println!("Listing tasks."),
        Command::Done(index)=>println!("Completing task: {index}"),
    };
}

fn parse_command(args: Vec<String>) -> Result<Command,String> {
    match args.first() {
     None=> Err(String::from("Missing Command")),
     Some(command_name) => {
        if command_name == "list" {
            Ok(Command::List)
        } else if command_name == "add" {
         match args.get(1) {
            None => Err(String::from("Missing task text")),
            Some(task)=> Ok(Command::Add(task.clone()))
         }
        } else if command_name == "done" {
            match args.get(1) {
                None => Err(String::from("Missing task number")),
                Some(task_text) => match task_text.parse::<usize>() {
                    Ok(task_number)=> {
                        if 1 <= task_number {
                        Ok(Command::Done(task_number))
                        }else {
                            Err(String::from("Task number out of bounds"))
                        }
                    },
                    Err(error)=> Err(format!("Error while trying to parse task number: {error}"))
                }
            }
        }
        else {
            Err(String::from("Unknown Command Error"))
        }
     }
    }
}