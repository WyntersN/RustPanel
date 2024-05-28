use crate::api::auth::AuthUser;
use crate::api::{ResponseStructure, ResponseStructureError};
use crate::models::file::{self, SaveData};

use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListData {
    pub path: String,
    pub current: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}
pub async fn list(_: AuthUser, data: web::Query<ListData>) -> HttpResponse {
    HttpResponse::Ok().json(ResponseStructure {
        success: true,
        code: 200,
        message: String::from("success"),
        data: Some(file::get_files_and_dirs_list(data.path.as_str())),
    })
}

pub async fn save(_: AuthUser, data: web::Json<SaveData>) -> HttpResponse {
    match file::save(data.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(ResponseStructure {
            success: true,
            code: 200,
            message: String::from("success"),
            data: Some("ok"),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseStructureError {
            success: false,
            code: 500,
            message: err.to_string(),
        }),
    }
}
#[derive(Deserialize)]
pub struct ContentData {
    pub path: String,
}
pub async fn content(_: AuthUser, data: web::Query<ContentData>) -> HttpResponse {
    match file::content(data.path.to_string()) {
        Ok((content, extension)) => HttpResponse::Ok().json(ResponseStructure {
            success: true,
            code: 200,
            message: String::from("Success"),
            data: Some(serde_json::json!({"content":content,"ext":extension})),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ResponseStructureError {
            success: false,
            code: 500,
            message: err.to_string(),
        }),
    }
}
