use serde::{Deserialize, Serialize};

use super::RestPublicUser;

#[derive(Deserialize)]
pub struct RestUserRegistrationRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RestUserRegistrationResponse {
    pub user: RestPublicUser,
    pub token: String
}