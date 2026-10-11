fn show_task(task: &str) {
    println!("{task}");
}

fn main() {
    let task = String::from("pay rent");
    show_task(&task);
    show_task("buy milk");
    println!("{task}");
}