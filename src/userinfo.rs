#[derive(Serialize, Deserialize)]
pub struct UserKey(pub u64);

#[derive(Serialize)]
pub struct UserInfo {
    pub id: UserKey,
    pub name: String
}

#[derive(Serialize)]
pub struct AuthorInfo {
    #[serde(flatten)]
    pub user_info: UserInfo,
    pub country: String
}

#[derive(Serialize)]
pub struct AuthorContacts {
    pub name: String,
    pub email: String
}