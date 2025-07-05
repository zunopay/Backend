use crate::db::entity::user::Model as UserModel;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    id: i32,
    email: String,
}

impl From<UserModel> for UserDto {
    fn from(value: UserModel) -> Self {
        UserDto {
            id: value.id,
            email: value.email,
        }
    }
}
