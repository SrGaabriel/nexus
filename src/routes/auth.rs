use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use std::sync::Arc;

use crate::{
    database,
    serializables::{
        auth::{RestUserRegistrationRequest, RestUserRegistrationResponse, RestUserLoginRequest, RestUserLoginResponse},
        RestPublicUser, RestSelfUser,
    },
    AppState,
};

#[post("/api/v1/signup")]
async fn signup(
    app_state: web::Data<AppState>,
    registration: Json<RestUserRegistrationRequest>,
) -> impl Responder {
    let common_user =
        database::user::search_by_email(app_state.rbatis.as_ref(), &registration.email)
            .await
            .ok();
    match common_user {
        Some(_) => HttpResponse::Conflict().finish(),
        None => {
            let user = database::user::new(
                app_state.rbatis.as_ref(),
                &app_state.snowflake.generate().unsigned_abs(),
                &registration.name,
                &registration.email,
                &registration.password,
                &0,
            )
            .await
            .unwrap();
            HttpResponse::Created().json(RestUserRegistrationResponse {
                token: app_state.authenticator.create_token(&user.id.unwrap()),
                user: RestPublicUser::from(user),
            })
        }
    }
}

#[post("/api/v1/login")]
async fn login(app_state: web::Data<AppState>, login: Json<RestUserLoginRequest>) -> impl Responder {
    let found_user = database::user::search_by_email(app_state.rbatis.as_ref(), &login.email).await.ok();
    if found_user.is_none() {
        return HttpResponse::NotFound().finish()
    }
    let unwrapped_found_user = found_user.unwrap();
    if unwrapped_found_user.password.as_ref().unwrap() != &login.password {
        return HttpResponse::Unauthorized().finish()
    } else {
        return HttpResponse::Ok().json(RestUserLoginResponse {
            token: app_state.authenticator.create_token(&unwrapped_found_user.id.unwrap()),
            user: RestSelfUser::from(unwrapped_found_user)
        })
    }
}