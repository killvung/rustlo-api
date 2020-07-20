use actix_web::get;
use actix_web::{HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rustlo world!")
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Rustlo!")
}

#[get("/again")]
pub async fn again() -> impl Responder {
    HttpResponse::Ok().body("Rustlo again!")
}
