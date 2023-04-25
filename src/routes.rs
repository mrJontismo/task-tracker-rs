use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::{info};
use serde_json::json;
use std::sync::Mutex;

use crate::AppState;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/index.html"))
}

#[get("/favicon.ico")]
pub async fn favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(include_bytes!("../templates/favicon.ico").as_ref())
}

#[get("/tasks_completed")]
pub async fn get_tasks_completed(state: web::Data<Mutex<AppState>>) -> impl Responder {
    let state = state.lock().unwrap();
    let jon_tasks_completed = state.jon.tasks_completed;
    let robin_tasks_completed = state.robin.tasks_completed;
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "jon": jon_tasks_completed,
            "robin": robin_tasks_completed,
        }))
}


#[post("/increment/{person}")]
pub async fn increment(state: web::Data<Mutex<AppState>>, person: web::Path<String>) -> impl Responder {
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

    

    info!("Incremented for user {}. New value: {}", person_to_update.name, person_to_update.tasks_completed);

    HttpResponse::Ok()
    .content_type("application/json")
    .json(json!({
        "name": &person_to_update.name,
        "tasks_completed": person_to_update.tasks_completed,
    }))
}

#[post("/decrement/{person}")]
pub async fn decrement(state: web::Data<Mutex<AppState>>, person: web::Path<String>) -> impl Responder {
    let mut state = state.lock().unwrap();
    let person_to_update = match person.into_inner().as_str() {
        "jon" => &mut state.jon,
        "robin" => &mut state.robin,
        _ => return HttpResponse::BadRequest().finish(),
    };

    person_to_update.decrement_tasks();

    info!("Decremented for user {}. New value: {}", person_to_update.name, person_to_update.tasks_completed);

    HttpResponse::Ok()
    .content_type("application/json")
    .json(json!({
        "name": &person_to_update.name,
        "tasks_completed": person_to_update.tasks_completed,
    }))
}