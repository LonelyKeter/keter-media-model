use serde::{Serialize, Deserialize}; 
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
