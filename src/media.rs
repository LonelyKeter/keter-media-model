use crate::*;

#[derive(Serialize, Deserialize)]
pub struct MediaKey(pub u64);

#[derive(Serialize)]
pub struct MediaInfo {
  pub id: MediaKey,
  pub title: String,
  pub kind: MediaKind,
  pub author: userinfo::AuthorInfo,
}

#[derive(Serialize, Deserialize)]
pub enum MediaKind {
  Audio,
  Video,
  Image
}

#[derive(Serialize)]
pub struct Media {
  pub info: MediaInfo,
  pub materials: Vec<MaterialInfo>
}

#[derive(Serialize, Deserialize, Default)]
pub struct MaterialKey(pub u64);

#[derive(Serialize, Deserialize)]
pub struct MaterialInfo {
  pub id: MaterialKey,
  pub format: String,
  pub quality: MaterialQuality,
  #[serde(rename = "licenseName")]
  pub license_name: String,
  #[serde(rename = "downloadLink")]
  pub download_link: String
}

//TODO: Material quality enumeration
#[derive(Serialize, Deserialize)]
pub enum MaterialQuality {
    Low,
    Medium,
    High
}

#[derive(Serialize, Deserialize)]
pub struct MediaRating(pub f64);

#[derive(Serialize)]
pub struct Tag {
    pub title: String,
    pub popularity: MediaRating
}

#[derive(Serialize, Deserialize)]
pub struct ReviewRating(pub u8);

#[derive(Deserialize)]
pub struct ReviewKey(pub u8);

#[derive(Serialize)]
pub struct UserReview {
    #[serde(rename = "userInfo")]
    pub user_info: userinfo::UserInfo,
    pub review: Review
}

#[derive(Serialize, Deserialize)]
pub struct Review {
    pub rating: ReviewRating,
    #[serde(skip_serializing_if = "Option::is_some")]
    pub text: Option<String> 
}

#[derive(Deserialize)]
pub struct RemoveReview {
    pub id: ReviewKey,
    pub reason: ReviewRemovalReasonKey
}

#[derive(Serialize, Deserialize)]
pub struct ReviewRemovalReasonKey(u32);

#[derive(Serialize, Deserialize)]
pub struct ReviewRemovalReason {
    pub id: ReviewRemovalReasonKey,
    pub statement: String
}

#[derive(Deserialize)]
pub struct RegisterMedia {
    pub title: String,
    pub kind: MediaKind,
    pub tags: Vec<String>,
    #[serde(rename = "defaultLicense")]
    pub default_license: Option<String>,
}

