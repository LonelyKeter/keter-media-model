use crate::*;

pub type MediaKey = i64;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MediaInfo {
  pub id: MediaKey,
  pub title: String,
  pub kind: MediaKind,
  pub author: userinfo::AuthorInfo,
  pub rating: MediaRating
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MediaKind {
  Audio,
  Video,
  Image
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Media {
  pub info: MediaInfo,
  pub materials: Vec<MaterialInfo>
}

pub type MaterialKey = i64;

#[derive(Debug)]
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

#[derive(Debug)]
//TODO: Material quality enumeration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MaterialQuality {
    Low,
    Medium,
    High
}

pub type MediaRating = f64;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Tag {
    pub title: String,
    pub popularity: MediaRating
}

pub type ReviewRating = i16;

pub type ReviewKey = i64;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserReview {
  #[cfg_attr(feature = "serde", serde(rename = "userInfo"))]
    pub user_info: userinfo::UserInfo,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub review: ReviewInfo
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReviewInfo {  
  pub id: ReviewKey;
    pub rating: ReviewRating,
    pub text: Option<String> 
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RemoveReview {
    pub id: ReviewKey,
    pub reason: ReviewRemovalReasonKey
}

pub type ReviewRemovalReasonKey = i32;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReviewRemovalReason {
    pub id: ReviewRemovalReasonKey,
    pub statement: String
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterMedia {
    pub title: String,
    pub kind: MediaKind,
    pub tags: Vec<String>,
    #[cfg_attr(feature = "serde", serde(rename = "defaultLicense"))]
    pub default_license: Option<String>,
}

