use std::sync::Arc;

use crate::database;
use crate::serializables::{RestPublicUser};
use actix_web::{get, web, HttpResponse, Responder};
use rbatis::rbatis::Rbatis;

#[get("/api/users/{id}")]
async fn get_user(rbatis: web::Data<Arc<Rbatis>>, id: web::Path<i64>) -> impl Responder {
    let local_user = database::user::search(&rbatis.into_inner(), &id.into_inner()).await;
    let response = match local_user {
        Result::Err(_) => HttpResponse::NotFound().finish(),
        Result::Ok(user) => HttpResponse::Ok().json(RestPublicUser::from(user)),
    };
    response
}