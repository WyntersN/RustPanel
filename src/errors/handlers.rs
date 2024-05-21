/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 18:16:54
 * @LastEditTime: 2024-05-10 19:21:48
 * @FilePath: \rust_panel\src\errors\handlers.rs
 */
use actix_web::{
    error::Error, error::JsonPayloadError, error::PathError, error::ResponseError,
    http::StatusCode, HttpRequest, HttpResponse,
};
use serde::Serialize;
use serde_json::json;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code)
            .json(json!({"code":self.error_status_code, "message": error_message }))
    }
}

// Custom JSON error handler for the JSON deserialization
// TODO: See if a user friendly version of the exact error can be generated (E.g. invalid type: string "1", expected u32)
pub fn json_error_handler(_err: JsonPayloadError, _req: &HttpRequest) -> Error {
    CustomError::new(400, format!("Invalid JSON payload")).into()
}

// Custom Path error handler for when the provided type in the URL does not match the expected type
pub fn path_error_handler(_err: PathError, _req: &HttpRequest) -> Error {
    CustomError::new(400, format!("Invalid URL path variables")).into()
}
