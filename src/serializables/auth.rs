use serde::{Deserialize, Serialize};

use super::{RestPublicUser, RestSelfUser};

#[derive(Deserialize)]
pub struct RestUserRegistrationRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RestUserRegistrationResponse {
    pub user: RestPublicUser,
    pub token: String,
}

#[derive(Deserialize)]
pub struct RestUserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RestUserLoginResponse {
    pub user: RestSelfUser,
    pub token: String,
}
