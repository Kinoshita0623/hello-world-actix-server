use crate::models::*;


pub trait UserRepository {
    fn find(&self, id: &i64) -> Result<User, diesel::result::Error>;
    fn delete(&self, id: &i64) -> bool;
    fn create(&self, u: NewUser) -> Result<User, diesel::result::Error>;
    fn message(&self) -> String;
}
