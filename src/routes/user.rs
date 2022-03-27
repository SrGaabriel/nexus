use std::sync::Arc;

use crate::routes::required_authentication;
use crate::serializables::{RestPublicUser, RestSelfUser};
use crate::{database, AppState};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/api/v1/users/{id}")]
async fn get_user(
    app_state: web::Data<AppState>,
    id: web::Path<u64>,
) -> impl Responder {
    let local_user = database::user::search(app_state.rbatis.as_ref(), &id.into_inner()).await;
    let response = match local_user {
        Result::Err(_) => HttpResponse::NotFound().finish(),
        Result::Ok(user) => HttpResponse::Ok().json(RestPublicUser::from(user)),
    };
    response
}

#[get("/api/v1/users/myself")]
async fn get_myself(app_state: web::Data<AppState>, request: HttpRequest) -> impl Responder {
    required_authentication(&request, app_state.as_ref(), |user| {
        HttpResponse::Ok().json(RestSelfUser::from(user))
    })
    .await
}