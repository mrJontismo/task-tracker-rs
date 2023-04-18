use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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
async fn index(state: web::Data<Mutex<AppState>>) -> impl Responder {
    let state = state.lock().unwrap();
    let jon = &state.jon;
    let robin = &state.robin;

    HttpResponse::Ok().body(format!(
        "{} has completed {} tasks. {} has completed {} tasks.",
        jon.name, jon.tasks_completed, robin.name, robin.tasks_completed
    ))
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
    HttpResponse::Ok().finish()
}

#[post("/decrement/{person}")]
async fn decrement(state: web::Data<Mutex<AppState>>, person: web::Path<String>) -> impl Responder {
    let mut state = state.lock().unwrap();
    let person_to_update = match person.into_inner().as_str() {
        "jon" => &mut state.jon,
        "robin" => &mut state.robin,
        _ => return HttpResponse::BadRequest().finish(),
    };

    person_to_update.decrement_tasks();
    HttpResponse::Ok().finish()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Mutex::new(AppState {
        jon: Person::new("Jon".to_string()),
        robin: Person::new("Robin".to_string()),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(increment)
            .service(decrement)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
