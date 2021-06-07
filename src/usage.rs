use crate::*;
use chrono::{DateTime, offset::Utc};

#[derive(Serialize)]
pub struct Usage {
    #[serde(rename = "materialId")]
    pub material_id: media::MaterialKey,
    #[serde(rename = "userName")] 
    pub user_name: String,
    pub email: String,
    pub date: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct LicenseInfo {
    pub title: String,
    pub link: String
}

#[derive(Serialize, Deserialize)]
pub struct LicenseKey(pub u32);

#[derive(Serialize, Deserialize)]
pub struct License {
    pub id: LicenseKey,
    #[serde(flatten)]
    pub info: LicenseInfo
}