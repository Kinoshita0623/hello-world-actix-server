use crate::models::*;
use std::vec::Vec;

pub trait UserRepository {
    fn find(&self, id: &i64) -> Result<User, diesel::result::Error>;
    fn delete(&self, id: &i64) -> bool;
    fn create(&self, u: NewUser) -> Result<User, diesel::result::Error>;
    fn message(&self) -> String;
    fn find_all(&self) -> Result<Vec<User>, diesel::result::Error>;
}
