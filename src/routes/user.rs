use std::sync::Arc;

use crate::routes::required_authentication;
use crate::{database, AppState};
use crate::serializables::{RestPublicUser, RestSelfUser};
use actix_web::{get, web, HttpResponse, Responder, HttpRequest};

#[get("/api/v1/users/{id}")]
async fn get_user(app_state: web::Data<Arc<AppState>>, request: HttpRequest, id: web::Path<String>) -> impl Responder {
    let string_id = id.into_inner();
    match string_id.as_str() {
        "myself" => {
            required_authentication(&request, app_state.as_ref(), |user| {
                HttpResponse::Ok().json(RestSelfUser::from(user))
            }).await
        },
        _ => {
            let integer_id = string_id.parse::<u64>();
            if integer_id.is_err() {
                return HttpResponse::NotFound().body(format!("can not parse {string_id} to a u64"))
            }
            let local_user = database::user::search(app_state.rbatis.as_ref(), &integer_id.unwrap()).await;
            let response = match local_user {
                Result::Err(_) => HttpResponse::NotFound().finish(),
                Result::Ok(user) => HttpResponse::Ok().json(RestPublicUser::from(user)),
            };
            response
        }
    }
}

// #[get("/api/v1/users/myself")]
// async fn get_myself(app_state: web::Data<Arc<AppState>>, request: HttpRequest) -> impl Responder {
//     required_authentication(&request, app_state.as_ref(), |user| {
//         HttpResponse::Ok().json(RestSelfUser::from(user))
//     }).await
// }