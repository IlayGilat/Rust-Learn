fn main() {
    let contents = String::from("buy milk\npay rent\nsend email\n");
    let mut tasks: Vec<String> = Vec::new();

    for line in contents.lines() {
        tasks.push(String::from(line));
    }
    tasks.remove(1);
    for line in &tasks {
        println!("{line}",);
    }
}
