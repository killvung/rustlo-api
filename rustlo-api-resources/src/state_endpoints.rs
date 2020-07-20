use actix_web::{get, web, Responder};
use std::sync::Mutex;

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub struct AppState {
    pub app_name: String,
}

#[get("/stateCounter")]
pub async fn state_counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[get("/")]
pub async fn state(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Rustlo {}!", app_name) // <- response with app_name
}
