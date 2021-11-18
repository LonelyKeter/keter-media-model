use crate::*;

pub type MediaKey = i64;
pub type Rating = f32;
pub type Count = i64;

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
    pub rating: Option<Rating>,
    #[cfg_attr(feature = "postgres", row(rename = "UseCount"))]
    pub use_count: Count
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSql, ToSql))]
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
    #[cfg_attr(feature = "serde", serde(rename = "mediaId"))]
    #[cfg_attr(feature = "postgres", row(rename = "MediaId"))]
    pub media_id: MediaKey,
    #[cfg_attr(feature = "postgres", row(rename = "Format"))]
    pub format: String,
    #[cfg_attr(feature = "postgres", row(rename = "Quality"))]
    pub quality: Quality,
    #[cfg_attr(feature = "serde", serde(rename = "licenseName"))]
    #[cfg_attr(feature = "postgres", row(rename = "LicenseName"))]
    pub license_name: String,
    #[cfg_attr(feature = "postgres", row(rename = "Rating"))]
    pub rating: Option<Rating>,
    #[cfg_attr(feature = "postgres", row(rename = "UseCount"))]
    pub use_count: Count,    
    #[cfg_attr(feature = "serde", serde(rename = "isUsed"))]
    #[cfg_attr(feature = "postgres", row(rename = "IsUsed"))]
    pub is_used: Option<bool>
}

#[derive(Debug, PartialEq)]
//TODO: Material quality enumeration
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSql, ToSql))]
#[cfg_attr(feature = "postgres", postgres(name = "quality"))]
pub enum Quality {    
    #[cfg_attr(feature = "serde", serde(rename = "Very low"))]
    #[cfg_attr(feature = "postgres", postgres(name = "VERY LOW"))]
    VeryLow,
    #[cfg_attr(feature = "postgres", postgres(name = "LOW"))]
    Low,
    #[cfg_attr(feature = "postgres", postgres(name = "MEDIUM"))]
    Medium,
    #[cfg_attr(feature = "postgres", postgres(name = "HIGH"))]
    High,
    #[cfg_attr(feature = "serde", serde(rename = "Very high"))]
    #[cfg_attr(feature = "postgres", postgres(name = "VERY HIGH"))]
    VeryHigh
}


#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Tag {
    pub title: String,
    pub popularity: Rating,
}

pub type ReviewRating = i16;
pub type ReviewKey = i64;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
#[cfg_attr(feature = "postgres", row(split))]
pub struct UserReview {
    #[cfg_attr(feature = "serde", serde(rename = "userInfo"))]
    #[cfg_attr(feature = "postgres", row(flatten, split = "id"))]
    pub user_info: userinfo::UserInfo,
    #[cfg_attr(feature = "serde", serde(flatten))] 
    #[cfg_attr(feature = "postgres", row(flatten, split = "id"))]
    pub review: ReviewInfo,
}

use chrono::{DateTime, offset::FixedOffset};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct ReviewInfo {    
    #[cfg_attr(feature = "postgres", row(rename = "Id"))]
    pub id: ReviewKey,    
    #[cfg_attr(feature = "postgres", row(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))] 
    pub review: Review,
    #[cfg_attr(feature = "postgres", row(rename = "Date"))]
    pub date: DateTime<FixedOffset>
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(FromSqlRow))]
pub struct Review {
    #[cfg_attr(feature = "postgres", row(rename = "Text"))]
    pub text: String
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


#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserRating {
    pub rating: i16
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
                media_id: 1,
                format: String::from(".jpeg"),
                quality: Quality::Medium,
                license_name: String::from("FREE"),
            };

            let query = query!("
                SELECT 
                    1::Int8 as Id, 1::Int8 as MediaId, '.jpeg' as Format, 
                    'MEDIUM'::public.quality as Quality,
                    'FREE' as LicenseName;"
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
                media_id: 1,
                format: String::from(".jpeg"),
                quality: Quality::VeryLow,
                license_name: String::from("FREE"),
            };
            let expected2 = MaterialInfo {
                id: 2,
                media_id: 1,
                format: String::from(".png"),
                quality: Quality::VeryHigh,
                license_name: String::from("FREE"),
            };
            let expected = vec![expected1, expected2];

            let query = query!("
                SELECT * FROM
                    (VALUES
                        (
                            1::Int8, 
                            1::Int8, 
                            '.jpeg', 
                            'VERY LOW'::public.quality,
                            'FREE'
                        ),
                        (
                            2::Int8,                             
                            1::Int8, 
                            '.png', 
                            'VERY HIGH'::public.quality,
                            'FREE'
                        )
                    ) x 
                    (Id, MediaId, Format, Quality, LicenseName);"
            );
            let rows = query.query(&client).await.unwrap();
            let queried = MaterialInfo::from_row_multi(&rows).unwrap();            

            for info in expected {
                assert!(queried.contains(&info));
            }
        }    

        #[tokio::test]
        async fn user_review_once() {
            use chrono::{TimeZone};
            let client = client().await.unwrap();

            let expected = UserReview {
                user_info: userinfo::UserInfo {
                    id: 5,
                    name: String::from("First user")
                },
                review: ReviewInfo {
                    id: 1,
                    review: Review { 
                        rating: 6, 
                        text: Some(String::from("Not so bad"))
                    },
                    date: FixedOffset::east(2 * 3600).ymd(2020, 12, 8).and_hms(7, 7, 7)
                }
            };

            let query = query!("
                SELECT 
                    5::Int8 as Id, 'First user' as Name, 
                    1::Int8 as Id, 6::Int2 as Rating, 'Not so bad' as Text,
                    '2020-12-08 07:07:07'::Timestamptz as Date;"
            );
            let row = query.query_one(&client).await.unwrap();
            let queried = UserReview::from_row(&row).unwrap();

            assert_eq!(expected, queried);
        }
    }
}
