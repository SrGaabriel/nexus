use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use std::sync::Arc;

use crate::{
    database,
    serializables::{
        auth::{RestUserRegistrationRequest, RestUserRegistrationResponse},
        RestPublicUser,
    },
    AppState,
};

#[post("/api/v1/signup")]
async fn signup(
    app_state: web::Data<Arc<AppState>>,
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
