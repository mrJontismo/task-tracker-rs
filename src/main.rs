use actix_web::{web, App, HttpServer};
use tracing::{info, metadata::LevelFilter};
use std::{sync::Mutex, fs};

mod models;
use crate::models::Person;

mod routes;
use crate::routes::*;

pub struct AppState {
    jon: Person,
    robin: Person,
}

fn read_tasks_completed_from_file(person_name: &str) -> u8 {
    let filename = format!("/app/data/{}.txt", person_name);
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(Mutex::new(AppState {
        jon: initialize_person_from_file("Jon"),
        robin: initialize_person_from_file("Robin"),
    }));

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let address = "0.0.0.0:8080";

    info!("Server running on {}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(get_tasks_completed)
            .service(favicon)
            .service(increment)
            .service(decrement)
    })
    .bind(address)?
    .run()
    .await
}
