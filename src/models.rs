pub struct Person {
    pub name: String,
    pub tasks_completed: u8,
    pub all_tasks_completed: bool,
}

impl Person {
    pub fn increment_tasks(&mut self) {
        self.tasks_completed += 1;
        if self.tasks_completed == 20 {
            self.all_tasks_completed = true;
        }
        self.write_tasks_completed_to_file();
    }

    pub fn decrement_tasks(&mut self) {
        if self.tasks_completed > 0 {
            self.tasks_completed -= 1;
            self.all_tasks_completed = false;
        }
        self.write_tasks_completed_to_file();
    }

    pub fn reset_tasks(&mut self) {
        self.tasks_completed = 0;
        self.all_tasks_completed = false;
        self.write_tasks_completed_to_file();
    }

    pub fn write_tasks_completed_to_file(&self) {
        let filename = format!("/app/data/{}.txt", self.name);
        if let Err(e) = std::fs::write(&filename, format!("{}", self.tasks_completed)) {
            eprintln!("Error writing file {}: {}", filename, e);
        }
    }
}