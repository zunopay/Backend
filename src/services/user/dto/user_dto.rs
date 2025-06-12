use crate::db::entity::User;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct UserDto {
    id: i32,
    username: String,
    email: String,
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        UserDto {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}
