use crate::repositories::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use crate::dao::*;
use crate::service::UserService;
use crate::service::PsqlUserService;

#[derive(Clone)]
pub struct AppState {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
}


impl AppState {
    pub fn user_repository(&self) -> impl UserRepository {
        return UserDAO {
            pool: self.pool.clone(),
            app_state: Box::new(self.clone())
        }
    }

    pub fn user_service(&self) -> impl UserService {
        return PsqlUserService {
            pool: self.pool.clone(),
            app_state: Box::new(self.clone())
        }
    }
}
