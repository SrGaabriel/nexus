mod auth;
mod config;
mod database;
mod routes;
mod serializables;

use std::{sync::Arc, net::{SocketAddr}};

use actix_web::{middleware::Logger, web, App, HttpServer};
use auth::Authenticator;
use config::Config;
use rbatis::{rbatis::Rbatis, snowflake::Snowflake};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    fast_log::init(fast_log::config::Config::new().console()).expect("logger init failed");
    log::info!("initiated logging");
    let config = config::load();
    log::info!("successfully loaded config!");

    let rbatis: Rbatis = Rbatis::new();

    let database_path: String = config.database.assemble();
    let server_address: SocketAddr = format!("{}:{}", &config.ip, &config.port).parse().expect("couldn't resolve domain");

    rbatis
        .link(database_path.as_str())
        .await
        .expect("database connection failed");
    let rbatis = Arc::new(rbatis);
    log::info!("linking database successful!");
    let authenticator = Arc::new(Authenticator::new(config.secret.clone()));
    let config = Arc::new(config);

    let app_state = AppState {
        config: config,
        rbatis: rbatis,
        authenticator: authenticator,
        snowflake: Arc::new(Snowflake::default()),
    };
    let app_state = Arc::new(app_state);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.to_owned()))
            .service(routes::index)
            .service(routes::user::get_myself)
            .service(routes::user::get_user)
            .service(routes::auth::signup)
    })
    .bind(server_address)?
    .run()
    .await
}

#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    rbatis: Arc<Rbatis>,
    snowflake: Arc<Snowflake>,
    authenticator: Arc<Authenticator>,
}
