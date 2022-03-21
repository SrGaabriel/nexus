use serde::Serialize;
use crate::databases::user::{User};

#[derive(Serialize)]
pub struct RestPublicUser {
    pub id: i64
}

impl From<User> for RestPublicUser  {
    fn from(user: User) -> Self {
        RestPublicUser {
            id: user.id.unwrap()
        }
    }
}