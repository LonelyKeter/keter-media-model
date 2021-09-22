use crate::*;
use chrono::{offset::FixedOffset, DateTime};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct UserUsage {
    #[cfg_attr(feature = "serde", serde(rename = "materialId"))]
    #[cfg_attr(feature = "postgres", row(rename = "MaterialId"))]
    pub material_id: media::MaterialKey,
    pub date: DateTime<FixedOffset>,
}

pub enum LicenseSearchKey {
    Id(LicenseKey),
    Title(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct LicenseInfo {
    #[cfg_attr(feature = "postgres", row(rename = "Title"))]
    pub title: String,
    #[cfg_attr(feature = "postgres", row(rename = "Text"))]
    pub text: String,
    #[cfg_attr(feature = "postgres", row(rename = "Date"))]
    pub date: DateTime<FixedOffset>
}

pub type LicenseKey = i32;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct License {
    pub id: LicenseKey,
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "postgres", row(flatten))]
    pub info: LicenseInfo,
}
