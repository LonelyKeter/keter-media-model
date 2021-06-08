use std::num::NonZeroU64;

#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserKey(pub NonZeroU64);

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