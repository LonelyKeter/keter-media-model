use crate::{*, userinfo::{UserInfo}};
use chrono::{offset::FixedOffset, DateTime};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
#[cfg_attr(feature = "postgres", row(split))]
pub struct Usage {
    #[cfg_attr(feature = "serde", serde(rename = "materialId"))]
    pub material_id: media::MaterialKey,
    pub date: DateTime<FixedOffset>,
    pub rating: Option<i16>,
    #[cfg_attr(feature = "postgres", row(flatten, split = "id"))]
    pub license: License,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
#[cfg_attr(feature = "postgres", row(split))]
pub struct UserUsage {
    #[cfg_attr(feature = "postgres", row(flatten))]
    #[cfg_attr(feature = "postgres", row(split = "id"))]
    user: UserInfo,
    #[cfg_attr(feature = "postgres", row(flatten))]
    #[cfg_attr(feature = "postgres", row(split = "material_id"))]
    usage: Usage
}

pub enum LicenseSearchKey {
    Id(LicenseKey),
    Title(String),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct LicenseInfo {
    pub title: String,
    pub text: String,
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

#[cfg(test)]
mod tests {
    pub use super::*;

    #[cfg(feature = "postgres")]
    mod postgres {
        use super::*;
        use crate::{
            tests::{postgres::*, *},
            *,
        };
    }
}