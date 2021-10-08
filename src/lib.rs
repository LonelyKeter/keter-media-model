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
        i16, Type::INT2
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
