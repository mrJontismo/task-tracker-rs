use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::sync::Mutex;

struct Person {
    name: String,
    tasks_completed: u8,
    all_tasks_completed: bool,
}

impl Person {
    fn new(name: String) -> Self {
        Person {
            name,
            tasks_completed: 0,
            all_tasks_completed: false,
        }
    }

    fn increment_tasks(&mut self) {
        self.tasks_completed += 1;
        if self.tasks_completed == 20 {
            self.all_tasks_completed = true;
        }
    }

    fn decrement_tasks(&mut self) {
        if self.tasks_completed > 0 {
            self.tasks_completed -= 1;
            self.all_tasks_completed = false;
        }
    }

    fn reset_tasks(&mut self) {
        self.tasks_completed = 0;
        self.all_tasks_completed = false;
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
        jon: Person::new("Jon".to_string()),
        robin: Person::new("Robin".to_string()),
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
