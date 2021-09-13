use crate::*;

pub type MediaKey = i64;

impl SqlType for MediaKey {
    const SQL_TYPE: Type = Type::INT8;
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
#[cfg_attr(feature = "postgres", row(split))]
pub struct MediaInfo {
    #[cfg_attr(feature = "postgres", row(split = "id"))]
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: MediaKey,
    #[cfg_attr(feature = "postgres", row(rename = "Title"))]
    pub title: String,
    #[cfg_attr(feature = "postgres", row(rename = "Kind"))]
    pub kind: MediaKind,
    #[cfg_attr(feature = "postgres", row(flatten, split = "id"))]
    pub author: userinfo::AuthorInfo,
    #[cfg_attr(feature = "postgres", row(rename = "Rating"))]
    pub rating: MediaRating,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSql))]
#[cfg_attr(feature = "postgres", postgres(name = "mediakind"))]
pub enum MediaKind {
    Audio,
    Video,
    Image,
}

pub type MaterialKey = i64;
pub type MaterialSize = i64;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct MaterialInfo {
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: MaterialKey,
    #[cfg_attr(feature = "postgres", row(rename = "Format"))]
    pub format: String,
    #[cfg_attr(feature = "postgres", row(rename = "Quality"))]
    pub quality: Quality,
    #[cfg_attr(feature = "postgres", row(rename = "Size"))]
    pub size: MaterialSize,
    #[cfg_attr(feature = "serde", serde(rename = "licenseName"))]
    #[cfg_attr(feature = "postgres", row(rename = "LicenseName"))]
    pub license_name: String,
    #[cfg_attr(feature = "serde", serde(rename = "downloadLink"))]
    #[cfg_attr(feature = "postgres", row(rename = "DownloadLink"))]
    pub download_link: String,
}

#[derive(Debug, PartialEq)]
//TODO: Material quality enumeration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSql))]
#[cfg_attr(feature = "postgres", postgres(name = "quality"))]
pub enum Quality {    
    #[cfg_attr(feature = "postgres", postgres(name = "VERY LOW"))]
    VeryLow,
    #[cfg_attr(feature = "postgres", postgres(name = "LOW"))]
    Low,
    #[cfg_attr(feature = "postgres", postgres(name = "MEDIUM"))]
    Medium,
    #[cfg_attr(feature = "postgres", postgres(name = "HIGH"))]
    High,
    #[cfg_attr(feature = "postgres", postgres(name = "VERY HIGH"))]
    VeryHigh
}

pub type MediaRating = f32;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Tag {
    pub title: String,
    pub popularity: MediaRating,
}

pub type ReviewRating = i16;
pub type ReviewKey = i64;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct UserReview {
    #[cfg_attr(feature = "serde", serde(rename = "userInfo"))]
    #[cfg_attr(feature = "postgres", row(flatten))]
    pub user_info: userinfo::UserInfo,
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "postgres", row(flatten))]
    pub review: ReviewInfo,
}

use chrono::{DateTime, offset::Utc};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct ReviewInfo {
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: ReviewKey,    
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "postgres", row(flatten))]
    pub review: Review, 
    #[cfg_attr(feature = "serde", serde(rename = "date"))]
    #[cfg_attr(feature = "postgres", row(rename = "Date"))]
    pub date: DateTime<Utc>
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct Review {
    #[cfg_attr(feature = "postgres", row(rename = "Rating"))]
    pub rating: ReviewRating,
    #[cfg_attr(feature = "postgres", row(rename = "Text"))]
    pub text: Option<String>,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct RemoveReview {
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: ReviewKey,
    #[cfg_attr(feature = "postgres", row(rename = "Reason"))]
    pub reason: ReviewRemovalReasonKey,
}

pub type ReviewRemovalReasonKey = i32;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct ReviewRemovalReason {
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: ReviewRemovalReasonKey,
    #[cfg_attr(feature = "postgres", row(rename = "Statement"))]
    pub statement: String,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterMedia {
    pub title: String,
    pub kind: MediaKind,
    pub tags: Vec<String>,
    #[cfg_attr(feature = "serde", serde(rename = "defaultLicense"))]
    pub default_license: Option<String>,
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

        #[tokio::test]
        async fn media_info_once() {
            let client = client().await.unwrap();

            let expected = MediaInfo {
                id: 1,
                title: String::from("AAA"),
                kind: MediaKind::Video,
                author: userinfo::AuthorInfo {
                    user_info: userinfo::UserInfo {
                        id: 2,
                        name: String::from("BBB"),
                    },
                    country: String::from("UA"),
                },
                rating: 5.5f32,
            };            

            let query = query!("
                SELECT 
                    1::Int8 as Id, 'AAA' as Title, 'Video'::public.mediakind as Kind,
                    2::Int8 as Id, 'BBB' as Name, 'UA' as Country,
                    5.5::real as Rating;"
            );
            let row = query.query_one(&client).await.unwrap();
            let queried = MediaInfo::from_row(&row).unwrap();

            assert_eq!(expected, queried);
        }
        #[tokio::test]
        async fn media_info_multi() {
            let client = client().await.unwrap();

            let expected1 = MediaInfo {
                id: 1,
                title: String::from("AAA"),
                kind: MediaKind::Video,
                author: userinfo::AuthorInfo {
                    user_info: userinfo::UserInfo {
                        id: 2,
                        name: String::from("AAA"),
                    },
                    country: String::from("UA"),
                },
                rating: 5.5f32,
            };       
            let expected2 = MediaInfo {
                id: 2,
                title: String::from("BBB"),
                kind: MediaKind::Video,
                author: userinfo::AuthorInfo {
                    user_info: userinfo::UserInfo {
                        id: 2,
                        name: String::from("BBB"),
                    },
                    country: String::from("UA"),
                },
                rating: 5.6f32,
            };                
            let expected = vec![expected1, expected2];

            let query = query!("
                SELECT * FROM
                    (VALUES
                        (
                            1::Int8, 
                            'AAA', 
                            'Video'::public.mediakind,
                            2::Int8, 
                            'AAA', 
                            'UA',
                            5.5::real
                        ),
                        (
                            2::Int8, 
                            'BBB', 
                            'Video'::public.mediakind,
                            2::Int8, 
                            'BBB', 
                            'UA',
                            5.6::real
                        )
                    ) x 
                    (Id, Title, Kind, Id, Name, Country, Rating);"
            );
            let rows = query.query(&client).await.unwrap();
            let queried = MediaInfo::from_row_multi(&rows).unwrap(); 

            for info in expected {
                assert!(queried.contains(&info));
            }
        }
    
        #[tokio::test]
        async fn material_info_once() {
            let client = client().await.unwrap();

            let expected = MaterialInfo {
                id: 1,
                format: String::from(".jpeg"),
                quality: Quality::Medium,
                size: 12345,
                license_name: String::from("FREE"),
                download_link: String::from("http://donload.file.com/1")
            };

            let query = query!("
                SELECT 
                    1::Int8 as Id, '.jpeg' as Format, 
                    'MEDIUM'::public.quality as Quality,
                    12345::Bigint as Size, 
                    'FREE' as LicenseName, 'http://donload.file.com/1' as DownloadLink;"
            );
            let row = query.query_one(&client).await.unwrap();
            let queried = MaterialInfo::from_row(&row).unwrap();

            assert_eq!(expected, queried);
        }

        #[tokio::test]
        async fn material_info_multi() {
            let client = client().await.unwrap();

            let expected1 = MaterialInfo {
                id: 1,
                format: String::from(".jpeg"),
                quality: Quality::VeryLow,
                size: 12345,
                license_name: String::from("FREE"),
                download_link: String::from("http://donload.file.com/1")
            };
            let expected2 = MaterialInfo {
                id: 2,
                format: String::from(".png"),
                quality: Quality::VeryHigh,
                size: 54321,
                license_name: String::from("FREE"),
                download_link: String::from("http://donload.file.com/2")
            };
            let expected = vec![expected1, expected2];

            let query = query!("
                SELECT * FROM
                    (VALUES
                        (
                            1::Int8, 
                            '.jpeg', 
                            'VERY LOW'::public.quality,
                            12345::bigint,
                            'FREE', 
                            'http://donload.file.com/1'
                        ),
                        (
                            2::Int8, 
                            '.png', 
                            'VERY HIGH'::public.quality,
                            54321::bigint,
                            'FREE', 
                            'http://donload.file.com/2'
                        )
                    ) x 
                    (Id, Format, Quality, Size, LicenseName, DownloadLink);"
            );
            let rows = query.query(&client).await.unwrap();
            let queried = MaterialInfo::from_row_multi(&rows).unwrap();            

            for info in expected {
                assert!(queried.contains(&info));
            }
        }    
           
    }
}
