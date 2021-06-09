#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; 
#[cfg(feature = "postgres_query")]
use postgres_query::FromSqlRow;

pub mod media;
pub mod reviews;
pub mod userinfo;
pub mod usage;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
