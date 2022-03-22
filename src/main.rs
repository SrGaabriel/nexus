mod database;
mod routes;
mod serializables;
mod snowflake;

use std::{sync::Arc};

use actix_web::{middleware::Logger, App, HttpServer, web};
use rbatis::rbatis::Rbatis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fast_log::init(fast_log::config::Config::new().console()).expect("logger init failed");
    log::info!("initiated logging");
    let rbatis: Rbatis = Rbatis::new();
    rbatis
        .link("postgresql://postgres:rustyCat@localhost:5432/sleeper")
        .await
        .expect("database connection failed");
    let rbatis = Arc::new(rbatis);
    log::info!("linking database successful!");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(rbatis.to_owned()))
            .service(routes::index)
            .service(routes::user::get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}