#[macro_use] extern crate serde;
extern crate chrono;

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
