use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::fs;
use std::sync::Mutex;

fn read_tasks_completed_from_file(person_name: &str) -> u8 {
    let filename = format!("{}.txt", person_name);
    match fs::read_to_string(&filename) {
        Ok(s) => {
            if let Ok(num) = s.trim().parse::<u8>() {
                num
            } else {
                eprintln!("Error: invalid number in file {}", filename);
                0
            }
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            0
        }
    }
}

fn initialize_person_from_file(person_name: &str) -> Person {
    let tasks_completed = read_tasks_completed_from_file(person_name);
    Person {
        name: person_name.to_string(),
        tasks_completed,
        all_tasks_completed: tasks_completed == 20,
    }
}

struct Person {
    name: String,
    tasks_completed: u8,
    all_tasks_completed: bool,
}

impl Person {
    fn increment_tasks(&mut self) {
        self.tasks_completed += 1;
        if self.tasks_completed == 20 {
            self.all_tasks_completed = true;
        }
        self.write_tasks_completed_to_file();
    }

    fn decrement_tasks(&mut self) {
        if self.tasks_completed > 0 {
            self.tasks_completed -= 1;
            self.all_tasks_completed = false;
        }
        self.write_tasks_completed_to_file();
    }

    fn reset_tasks(&mut self) {
        self.tasks_completed = 0;
        self.all_tasks_completed = false;
        self.write_tasks_completed_to_file();
    }

    fn write_tasks_completed_to_file(&self) {
        let filename = format!("{}.txt", self.name);
        if let Err(e) = fs::write(&filename, format!("{}", self.tasks_completed)) {
            eprintln!("Error writing file {}: {}", filename, e);
        }
    }
}

struct AppState {
    jon: Person,
    robin: Person,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[post("/increment/{person}")]
async fn increment(state: web::Data<Mutex<AppState>>, person: web::Path<String>) -> impl Responder {
    let mut state = state.lock().unwrap();
    let person_to_update = match person.into_inner().as_str() {
        "jon" => &mut state.jon,
        "robin" => &mut state.robin,
        _ => return HttpResponse::BadRequest().finish(),
    };

    person_to_update.increment_tasks();
    if person_to_update.all_tasks_completed {
        person_to_update.reset_tasks();
    }

    println!("Incremented for user {}", person_to_update.name);

    HttpResponse::Ok()
    .content_type("application/json")
    .json(json!({
        "name": &person_to_update.name,
        "tasks_completed": person_to_update.tasks_completed,
    }))

}

#[post("/decrement/{person}")]
async fn decrement(state: web::Data<Mutex<AppState>>, person: web::Path<String>) -> impl Responder {
    let mut state = state.lock().unwrap();
    let person_to_update = match person.into_inner().as_str() {
        "jon" => &mut state.jon,
        "robin" => &mut state.robin,
        _ => return HttpResponse::BadRequest().finish(),
    };

    println!("Decremented for user {}", person_to_update.name);

    person_to_update.decrement_tasks();

    HttpResponse::Ok()
    .content_type("application/json")
    .json(json!({
        "name": &person_to_update.name,
        "tasks_completed": person_to_update.tasks_completed,
    }))

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let address = "127.0.0.1:8080";

    let app_state = web::Data::new(Mutex::new(AppState {
        jon: initialize_person_from_file("Jon"),
        robin: initialize_person_from_file("Robin"),
    }));    

    println!("Server running on {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(increment)
            .service(decrement)
    })
    .bind(address)?
    .run()
    .await
}
