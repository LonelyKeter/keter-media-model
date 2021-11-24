#[cfg(feature = "postgres")]
use postgres_query::FromSqlRow;
#[cfg(feature = "postgres")]
use postgres_types::{FromSql, ToSql};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "postgres")]
pub use sql_type::SqlType;

pub mod media;
pub mod reviews;
pub mod usage;
pub mod userinfo;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(ToSql))]
#[cfg_attr(feature = "postgres", postgres(name = "FILTER_ORDERING"))]
pub enum FilterOrdering {    
    #[cfg_attr(feature = "serde", serde(rename = "asc"))]
    #[cfg_attr(feature = "postgres", postgres(name = "asc"))]
    Ascending,
    #[cfg_attr(feature = "serde", serde(rename = "desc"))]    
    #[cfg_attr(feature = "postgres", postgres(name = "desc"))]
    Descending,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(ToSql))]
#[cfg_attr(feature = "postgres", postgres(name = "LIMITS"))]
pub struct Limits {
    pub min: Option<i64>,
    pub max: Option<i64>
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "postgres", derive(ToSql))]
#[cfg_attr(feature = "postgres", postgres(name = "RANGE_FILTER"))]
pub struct RangeFilter {
    pub ordering: Option<FilterOrdering>,
    pub limits: Limits
}

#[cfg(feature = "postgres")]
mod sql_type {
    use tokio_postgres::types::Type;

    pub trait SqlType {
        const SQL_TYPE: Type;
    }

    macro_rules! impl_sql_type {
        ($($t:ty, $repr:path),+) => {
            $(impl SqlType for $t {
                const SQL_TYPE: Type = $repr;
            })+
        };
    }

    impl_sql_type!(
        i64, Type::INT8,
        i32, Type::INT4,
        i16, Type::INT2,
        String, Type::VARCHAR
    );
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "postgres")]
    pub mod postgres {
        pub use postgres_query::query;
        use tokio_postgres::{Client, Config, Error};

        async fn establish_connection(config: &Config) -> Result<Client, Error> {
            use tokio_postgres::NoTls;

            let (client, connection) = config.connect(NoTls).await?;
            tokio::spawn(async move {
                connection.await;
            });

            Ok(client)
        }

        pub async fn client() -> Result<Client, Error> {
            establish_connection(
                Config::new()
                    .user("keter_media_test")
                    .password("keter_media_test")
                    .dbname("ketermedia")
                    .host("localhost")
                    .port(5432),
            )
            .await
        }

        #[tokio::test]
        async fn it_works() {
            client().await.unwrap();
        }
    }
}
