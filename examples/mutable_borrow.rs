fn main() {
    let mut task = String::from("pay rent");
    add_suffix(&mut task);

    let first = &mut task;
    add_suffix(first);
    let second = &mut task;
    add_suffix(second);
    println!("{}", task);
}

fn add_suffix(task: &mut String) {
    task.push_str(" today");
}
