use crate::*;

pub type MediaKey = u64;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MediaInfo {
  pub id: MediaKey,
  pub title: String,
  pub kind: MediaKind,
  pub author: userinfo::AuthorInfo,
  pub rating: MediaRating
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MediaKind {
  Audio,
  Video,
  Image
}

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Media {
  pub info: MediaInfo,
  pub materials: Vec<MaterialInfo>
}
pub type MaterialKey = u64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MaterialInfo {
  pub id: MaterialKey,
  pub format: String,
  pub quality: MaterialQuality,
  #[cfg_attr(feature = "serde", serde(rename = "licenseName"))]
  pub license_name: String,
  #[cfg_attr(feature = "serde", serde(rename = "downloadLink"))]
  pub download_link: String
}

//TODO: Material quality enumeration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MaterialQuality {
    Low,
    Medium,
    High
}


pub type MediaRating = f64;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Tag {
    pub title: String,
    pub popularity: MediaRating
}

pub type ReviewRating = u8;

pub type ReviewKey = u64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserReview {
  #[cfg_attr(feature = "serde", serde(rename = "userInfo"))]
    pub user_info: userinfo::UserInfo,
    pub review: Review
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Review {
    pub rating: ReviewRating,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub text: Option<String> 
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RemoveReview {
    pub id: ReviewKey,
    pub reason: ReviewRemovalReasonKey
}

pub type ReviewRemovalReasonKey = u32;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReviewRemovalReason {
    pub id: ReviewRemovalReasonKey,
    pub statement: String
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterMedia {
    pub title: String,
    pub kind: MediaKind,
    pub tags: Vec<String>,
    #[cfg_attr(feature = "serde", serde(rename = "defaultLicense"))]
    pub default_license: Option<String>,
}

