mod auth;
mod database;
mod routes;
mod serializables;

use std::sync::Arc;

use actix_web::{middleware::Logger, web, App, HttpServer};
use auth::Authenticator;
use rbatis::rbatis::Rbatis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fast_log::init(fast_log::config::Config::new().console()).expect("logger init failed");
    log::info!("initiated logging");
    let rbatis: Rbatis = Rbatis::new();
    rbatis
        .link("postgresql://postgres:password@localhost:5432/sleeper")
        .await
        .expect("database connection failed");
    let rbatis = Arc::new(rbatis);
    log::info!("linking database successful!");

    let app_state = AppState {
        authenticator: Arc::new(Authenticator::new("test_secret".to_string())),
        rbatis: rbatis,
    };
    let app_state = Arc::new(app_state);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.to_owned()))
            .service(routes::index)
            .service(routes::user::get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Clone)]
pub struct AppState {
    rbatis: Arc<Rbatis>,
    authenticator: Arc<Authenticator>,
}