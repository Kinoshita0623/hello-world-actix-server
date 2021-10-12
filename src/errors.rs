use diesel::result::Error;
use crate::HttpResponse;
use actix_web::http::StatusCode;
use serde::Serialize;

pub enum ServiceError {
    Client {
        message: String
    },
    Server {
        message: Option<String>
    },
    NotFound,
    Unauthorized
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub message: String
}

impl ServiceError {

    pub fn from_diesel_result_error(err: Error) -> Self {
        return match err {
            Error::NotFound => {
                ServiceError::NotFound
            }
            _ => ServiceError::Server {
                message: Some(String::from(err.to_string()))
            }
        };
    }

    pub fn response(&self) -> HttpResponse {
        return match self {
            ServiceError::Client { message: msg } => {
                HttpResponse::build(StatusCode::BAD_REQUEST).json(ErrorBody{
                    message: msg.to_string()
                })
            },
            Server => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(
                    ErrorBody{
                        message: "INTERNAL SERVER ERROR".to_string()
                    }
                )
            },
            NotFound => {
                HttpResponse::build(StatusCode::NOT_FOUND).json(
                    ErrorBody {
                        message: "NOT FOUND".to_string()
                    }
                )
            },
            Unauthorized => {
                HttpResponse::build(StatusCode::UNAUTHORIZED).json(
                    ErrorBody {
                        message: "UNAUTHORIZED".to_string()
                    }
                )
            }

        };
    }
}