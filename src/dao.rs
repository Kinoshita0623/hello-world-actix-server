use crate::repositories::*;
use diesel::pg::PgConnection;
use crate::schema::posts;
use crate::schema::users;
extern crate diesel;
use crate::models::*;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;


pub struct UserDAO {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>
}

struct PostDAO<'a> {
    pub connection: &'a PgConnection,
}

impl UserRepository for UserDAO {
    fn find(&self, id: &i64) -> Result<User, diesel::result::Error> {
        let connection = &self.pool.get().expect("error");
        return users::dsl::users.filter(users::id.eq(id)).first::<User>(connection);
    }

    fn delete(&self, id: &i64) -> bool {
        let connection = &self.pool.get().expect("error");
        return diesel::delete(users::dsl::users.filter(users::id.eq(id))).execute(connection).is_ok();
    }

    fn create(&self, u: NewUser) -> Result<User, diesel::result::Error> {
        let connection = &self.pool.get().expect("error");
        let user = diesel::insert_into(users::table)
            .values(u)
            .get_result(connection)?;
        return Ok(user);

            
    }

    fn message(&self) -> String {
        return String::from("DAOからなのだ");
    }
}

impl UserDAO {
    pub fn hoge(&self) -> String{
        return String::from("hoge");
    }
}