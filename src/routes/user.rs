use std::sync::Arc;

use crate::{database, AppState};
use crate::serializables::{RestPublicUser};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/v1/users/{id}")]
async fn get_user(app_state: web::Data<Arc<AppState>>, id: web::Path<u64>) -> impl Responder {
    let local_user = database::user::search(app_state.rbatis.as_ref(), &id.into_inner()).await;
    let response = match local_user {
        Result::Err(_) => HttpResponse::NotFound().finish(),
        Result::Ok(user) => HttpResponse::Ok().json(RestPublicUser::from(user)),
    };
    response
}