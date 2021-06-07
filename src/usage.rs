use crate::*;
use chrono::{DateTime, offset::Utc};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Usage {
    #[cfg_attr(feature = "serde", serde(rename = "materialId"))]
    pub material_id: media::MaterialKey,
    #[cfg_attr(feature = "serde", serde(rename = "userName"))]
    pub user_name: String,
    pub email: String,
    pub date: DateTime<Utc>
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LicenseInfo {
    pub title: String,
    pub link: String
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LicenseKey(pub u32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct License {
    pub id: LicenseKey,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub info: LicenseInfo
}