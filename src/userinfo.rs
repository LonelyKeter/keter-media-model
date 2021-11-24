use crate::*;

pub type UserKey = i64;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct UserInfo {
    pub id: UserKey,
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
    #[cfg_attr(feature = "serde", serde(rename = "isAuthor"))]
    pub is_author: bool,
    #[cfg_attr(feature = "serde", serde(rename = "administrationPermissions"))]    
    pub administration_permissions: AdministrationPermissions
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSql, ToSql))]
#[cfg_attr(feature = "postgres", postgres(name = "administration_permissions"))]
pub enum AdministrationPermissions {
    #[cfg_attr(feature = "postgres", postgres(name = "none"))]
    None,
    #[cfg_attr(feature = "postgres", postgres(name = "moderator"))]
    Moderator,
    #[cfg_attr(feature = "postgres", postgres(name = "admin"))]
    Admin
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
