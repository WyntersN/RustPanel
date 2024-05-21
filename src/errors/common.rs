/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-09 16:34:40
 * @LastEditTime: 2024-05-13 22:03:04
 * @FilePath: \RustPanel\src\errors\common.rs
 */
use std::{convert::From, io};

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};

use crate::api::ResponseStructureError;

#[derive(Debug, Display)]
pub enum CommonError {
    #[display(fmt = "Internal Server Error")]
    Error(io::Error),

    #[display(fmt = "Internal Server Error")]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {_0}")]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for CommonError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CommonError::Error(ref err) => HttpResponse::InternalServerError()
                .json(ResponseStructureError {
                    success: false,
                    code: 500,
                    message: err.to_string(),
                }),
            CommonError::InternalServerError(ref message)  => HttpResponse::InternalServerError()
                .json(ResponseStructureError {
                    success: false,
                    code: 500,
                    message: String::from(message),
                }),
            CommonError::BadRequest(ref message) => HttpResponse::BadRequest()
                .json(ResponseStructureError {
                    success: false,
                    code: 400,
                    message: String::from(message),
                }),
            CommonError::Unauthorized(ref message) => HttpResponse::Unauthorized()
                .json(ResponseStructureError {
                    success: false,
                    code: 401,
                    message: String::from(message),
                }),
        }
    }
}

impl From<DBError> for CommonError {
    fn from(error: DBError) -> CommonError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_owned();
                    return CommonError::BadRequest(message);
                }
                CommonError::InternalServerError(info.message().to_string())
            }
            _ => CommonError::InternalServerError(error.to_string()),
        }
    }
}
