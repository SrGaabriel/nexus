use serde::Serialize;
use crate::database::user::User;

pub mod auth;

#[derive(Serialize)]
pub struct RestPublicUser {
    pub id: u64,
    pub name: String,
    #[serde(alias = "type")]
    pub account_type: u8,
    pub biography: Option<String>,
    pub tagline: Option<String>
}

#[derive(Serialize)]
pub struct RestSelfUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    #[serde(alias = "type")]
    pub account_type: u8,
    pub biography: Option<String>,
    pub tagline: Option<String>
}

impl From<User> for RestPublicUser {
    fn from(user: User) -> Self {
        RestPublicUser {
            id: user.id.unwrap(),
            name: user.name.unwrap(),
            account_type: user.account_type.unwrap(),
            biography: user.biography,
            tagline: user.tagline,
        }
    }
}

impl From<User> for RestSelfUser {
    fn from(user: User) -> Self {
        RestSelfUser {
            id: user.id.unwrap(),
            name: user.name.unwrap(),
            email: user.email.unwrap(),
            account_type: user.account_type.unwrap(),
            biography: user.biography,
            tagline: user.tagline,
        }
    }
}