fn main() {
    let task_number = "2";
    let task_number = task_number.parse::<usize>().unwrap();
    let next_number: usize = { task_number + 1 };
    println!("{}", next_number);
    let completed = true;
    let label = if completed == true { "done" } else { "pending" };
    println!("{}", label)
}
