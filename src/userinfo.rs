use crate::*;

use std::num::NonZeroU64;

pub type UserKey = NonZeroU64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserInfo {
    pub id: UserKey,
    pub name: String
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthorInfo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub user_info: UserInfo,
    pub country: String
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthorContacts {
    pub name: String,
    pub email: String
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres_query", derive(FromSqlRow))]
pub struct UserPrivelegies {
    pub author: bool,
    pub moderator: bool,
    pub admin: bool
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres_query", derive(FromSqlRow))]
pub struct LoginData {
    pub email: String,
    pub password: String
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterData {
    #[serde(rename = "userName", flatten)]
    pub user_name: String,
    #[serde(rename = "loginData", flatten)]
    pub login_data: LoginData
}