use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::{database::user::*, AppState};

pub mod user;

#[get("/api")]
async fn index() -> impl Responder {
    format!("Welcome to the official sleeper API!")
}

pub async fn authenticate(request: &HttpRequest, app_state: &AppState) -> Option<User> {
    let user_id = request
        .headers()
        .get("Authorization")
        .map(|auth_header| auth_header.to_str().unwrap())
        .map(|token| app_state.authenticator.extract_user_id_from_token(token.to_string()))
        .flatten()?;
    search(app_state.rbatis.as_ref(), &user_id).await.ok()
}

pub async fn required_authentication<F>(request: &HttpRequest, app_state: &AppState, closure: F) -> HttpResponse 
where
    F: FnOnce(User) -> HttpResponse + 'static,
{
    let user = authenticate(request, app_state).await;
    match user {
        None => HttpResponse::Unauthorized().finish(),
        Some(user) => closure(user)
    }
}