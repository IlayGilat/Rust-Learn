fn main() {
    let task_number: Option<usize> = Some(2);
    if let Some(number) = task_number {
        println!("{}", number);
    }
    println!("Continuing");
}
