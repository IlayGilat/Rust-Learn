struct Task {
    text: String,
    completed: bool,
}

impl Task {
    fn new(text: String) -> Self {
        return Task {
            text,
            completed: false,
        };
    }

    fn text(&self) -> &str {
        return &self.text;
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn into_text(self) -> String {
        return self.text;
    }
}
fn main() {
    let mut my_task = Task::new(String::from("pay rent"));
    println!("{}", my_task.completed);
    my_task.complete();
    println!("{}", my_task.completed);
    let my_text = my_task.into_text();
    println!("{}", my_text);
}
