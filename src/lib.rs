#[cfg(feature = "postgres")]
use postgres_query::FromSqlRow;
#[cfg(feature = "postgres")]
use postgres_types::FromSql;
#[cfg(feature = "postgres")]
use tokio_postgres::types::Type;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod media;
pub mod reviews;
pub mod usage;
pub mod userinfo;

pub trait SqlType {
    const SQL_TYPE: Type;
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
