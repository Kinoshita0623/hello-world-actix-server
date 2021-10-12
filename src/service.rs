use crate::models::User;
use crate::errors::ServiceError;
use crate::schema::users;
use crate::schema::user_tokens;
use diesel::prelude::*;
use crate::models::*;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use crate::state::AppState;
use crate::repositories::UserRepository;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LogoutUser {
    pub token: String    
}

#[derive(Serialize)]
pub struct AuthResult {
    pub token: String,
    pub user: User,
}


pub trait UserService {
    fn login(&self, login_user: LoginUser) -> Result<AuthResult, ServiceError>;
    fn logout(&self, logout_user: LogoutUser) -> Result<(), ServiceError>;
    fn register(&self, register_user: RegisterUser) -> Result<AuthResult, ServiceError>;
}

pub struct PsqlUserService {
    pub pool: Box<Pool<ConnectionManager<PgConnection>>>,
    pub app_state: Box<AppState>
}

impl  UserService for PsqlUserService {
    fn login(&self, login_user: LoginUser) -> Result<AuthResult, ServiceError> {
        let connection = match self.pool.get() {
            Ok(connection) => connection,
            Err(_) => return Err(ServiceError::Server { message: None }),
        };

        let user = match users::dsl::users.filter(users::username.eq(login_user.username)).first::<User>(&connection) {
            Ok(user) => user,
            Err(err) => return Err(ServiceError::from_diesel_result_error(err))
        };
        
        if !user.check_password(login_user.password) {
            return Err(
                ServiceError::Client {
                    message: String::from("password not missmatch")
                }
            );
        }
        let token = NewUserToken::new(user.id);
        return match diesel::insert_into(user_tokens::table).values(token).get_result::<UserToken>(&connection) {
            Ok(user_token) => {
                Ok(
                    AuthResult {
                        token: user_token.token,
                            user: user
                        }
                )
            }
            Err(_) => {
                Err(
                    ServiceError::Server {
                        message: Some(String::from(""))
                    }
                )
            }
        };


    }

    fn logout(&self, logout_user: LogoutUser) -> Result<(), ServiceError> {
        if let Ok(connection) = self.pool.get() {
            return if diesel::delete(user_tokens::dsl::user_tokens.filter(user_tokens::token.eq(logout_user.token))).execute(&connection).is_ok() {
                Ok(())
            } else {
                Err(ServiceError::Server {
                    message: None
                })
            };
        }
        return Err(ServiceError::NotFound);
    }

    fn register(&self, register_user: RegisterUser) -> Result<AuthResult, ServiceError> {
        let connection = match self.pool.get() {
            Ok(connection) => connection,
            Err(_) => return Err(ServiceError::Server {
                message: Some(String::from(""))
            })
        };
        
        let new_user = match NewUser::new(register_user.username, register_user.password) {
            Ok(new_user) => new_user,
            Err(e) => return Err(
                ServiceError::Server {
                    message: Some(e)
                }
            )
        };
        let repo = &self.app_state.user_repository();

        let user = match repo.create(new_user) {
            Ok(user) => user,
            Err(e) => return Err(ServiceError::from_diesel_result_error(e))
        };
        let token = NewUserToken::new(user.id);
        let user_token = match diesel::insert_into(user_tokens::table).values(token).get_result::<UserToken>(&connection) {
            Ok(user_token) => user_token,
            Err(e) => return Err(ServiceError::from_diesel_result_error(e))
        };
        return Ok(AuthResult {
            token: user_token.token,
            user: user
        });
        
    }
    
}