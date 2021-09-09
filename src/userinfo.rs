use crate::*;

pub type UserKey = i64;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct UserInfo {
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: UserKey,
    #[cfg_attr(feature = "postgres", row(rename = "Name"))]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct AuthorInfo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "postgres", row(flatten))]
    pub user_info: UserInfo,
    pub country: String,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuthorContacts {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct UserPriveleges {
    #[cfg_attr(feature = "postgres", row(rename = "Author"))]
    pub author: bool,
    #[cfg_attr(feature = "postgres", row(rename = "Moderator"))]
    pub moderator: bool,
    #[cfg_attr(feature = "postgres", row(rename = "Administrator"))]
    pub admin: bool,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterData {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "loginData", flatten)]
    pub login_data: LoginData,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct UserIdPassHash {
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: UserKey,
    #[cfg_attr(feature = "postgres", row(rename = "Password"))]
    pub password_hash: Vec<u8>,
}
