use crate::*;

pub type UserKey = i32;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres_query", derive(FromSqlRow))]
pub struct UserInfo {
    #[cfg_attr(feature = "postgres_query", row(rename = "Id"))]
    pub id: UserKey,
    #[cfg_attr(feature = "postgres_query", row(rename = "Name"))]
    pub name: String
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthorInfo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub user_info: UserInfo,
    pub country: String
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthorContacts {
    pub name: String,
    pub email: String
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres_query", derive(FromSqlRow))]
pub struct UserPriveleges {
    #[cfg_attr(feature = "postgres_query", row(rename = "Author"))]
    pub author: bool,
    #[cfg_attr(feature = "postgres_query", row(rename = "Moderator"))]
    pub moderator: bool,
    #[cfg_attr(feature = "postgres_query", row(rename = "Administrator"))]
    pub admin: bool
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres_query", derive(FromSqlRow))]
pub struct LoginData {
    pub email: String,
    pub password: String
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterData {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "loginData", flatten)]
    pub login_data: LoginData
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "postgres_query", derive(FromSqlRow))]
pub struct UserIdPassHash {
    #[cfg_attr(feature = "postgres_query", row(rename = "Id"))]
    pub id: UserKey,
    #[cfg_attr(feature = "postgres_query", row(rename = "Password"))]
    pub password_hash: Vec<u8>
}