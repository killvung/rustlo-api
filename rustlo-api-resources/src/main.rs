use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod greeting_endpoints;
use greeting_endpoints::*;

mod state_endpoints;
use state_endpoints::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("rustlo-api"),
            })
            .app_data(counter.clone()) // <- register the created data
            .service(
                web::scope("/greetings")
                    .service(index)
                    .service(hello)
                    .service(again),
            )
            .service(web::scope("/states").service(state).service(state_counter))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
