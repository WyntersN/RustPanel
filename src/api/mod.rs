/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-09 18:54:56
 * @LastEditTime: 2024-05-12 19:55:52
 * @FilePath: \RustPanel\src\api\mod.rs
 */

pub mod auth;
pub mod route;
pub mod v1;
//pub mod middleware;

use actix_web::HttpResponse;
use serde::Serialize;
#[derive(Serialize)]
pub struct ResponseStructure<T> {
    pub success: bool,
    pub data: Option<T>,
    pub code: i16,
    pub message: String,
}
#[derive(Serialize)]
pub struct ResponseStructureError {
    pub success: bool,
    pub code: i16,
    pub message: String,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Forbidden().body("403 Forbidden")
}
