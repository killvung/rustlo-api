use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/stateCounter")]
async fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/state")]
async fn index4(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Rustlo {}!", app_name) // <- response with app_name
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Rust there!")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rustlo world!")
}

#[get("/again")]
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Rustlo world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("rustlo-world-api"),
            })
            .app_data(counter.clone()) // <- register the created data
            .service(
                web::scope("/index")
                    .service(index)
                    .service(index2)
                    .service(index3)
                    .service(index4)
                    .service(_index),
            )
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
