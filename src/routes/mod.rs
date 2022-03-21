use actix_web::{get, Responder};

pub mod user;

#[get("/")]
async fn index() -> impl Responder {
    format!("Welcome to the official sleeper API!")
}