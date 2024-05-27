use crate::api::auth::AuthUser;
use crate::api::{ResponseStructure, ResponseStructureError};
use crate::common::fun::{sm4_decrypt_file, sm4_encrypt_file};
use crate::service::global::CONF;
use actix_web::{web, Error, HttpResponse};
use encoding_rs::{GBK, UTF_8};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::time::UNIX_EPOCH;
#[derive(Deserialize)]
pub struct ListData {
    pub path: String,
    pub current: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}
#[derive(Deserialize)]
pub struct SaveData {
    pub path: String,
    pub content: String,
}

pub async fn list(_: AuthUser, data: web::Query<ListData>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(ResponseStructure {
        success: true,
        code: 200,
        message: String::from("success"),
        data: Some(get_files_and_dirs_list(data.path.as_str())),
    }))
}

pub async fn save(_: AuthUser, data: web::Json<SaveData>) -> Result<HttpResponse, Error> {
    println!("{}", data.path);

    //保存文件
    let content = sm4_decrypt_file(data.content.to_string());
    if let Err(e) = fs::write(&data.path, content) {
        return Ok(
            HttpResponse::InternalServerError().json(ResponseStructureError {
                success: false,
                code: 500,
                message: String::from(t!("file.save.write_error",error=>e.to_string())),
            }),
        );
    }

    Ok(HttpResponse::Ok().json(ResponseStructure {
        success: true,
        code: 200,
        message: String::from("success"),
        data: Some("ok"),
    }))
}
#[derive(Deserialize)]
pub struct ContentData {
    pub path: String,
}
pub async fn content(_: AuthUser, data: web::Query<ContentData>) -> HttpResponse {
    // 获取文件元数据
    let metadata = fs::metadata(&data.path.to_string()).map_err(|_| {
        // 如果无法获取元数据，返回错误
        return HttpResponse::InternalServerError().json(ResponseStructureError {
            success: false,
            code: 500,
            message: String::from(t!("file.content.info_error")),
        });
    });
    // 先定义一个外部变量，用于存储文件的扩展名
    let mut extension = String::new();

    if let Some(ext) = Path::new(&data.path).extension().and_then(OsStr::to_str) {
        extension = ext.to_string();
        if extension == "exe" || extension == "dll" || extension == "so" || extension == "zip" {
            return HttpResponse::InternalServerError().json(ResponseStructureError {
                success: false,
                code: 500,
                message: String::from(t!("file.content.file_type_error")),
            });
        }
    }

    // 检查文件大小是否超过100MB
    if metadata.unwrap().len() as f64 / 1048576.0 > CONF.app.max_file_size {
        // 如果文件大小超过10MB，返回错误
        return HttpResponse::InternalServerError().json(ResponseStructureError {
            success: false,
            code: 500,
            message: String::from(
                t!("file.content.file_max_size_error",size=>CONF.app.max_file_size),
            ),
        });
    }

    // 打开文件
    let mut file = match File::open(&data.path.to_string()) {
        Ok(file) => file,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ResponseStructureError {
                success: false,
                code: 500,
                message: String::from(t!("file.content.open_error")),
            });
        }
    };

    let content = match fs::read_to_string(data.path.to_string()) {
        Ok(content) => content,
        Err(_) => {
            // 读取文件内容到一个字节数组中
            let mut content_vec = Vec::new();
            if let Err(_) = file.read_to_end(&mut content_vec) {
                return HttpResponse::InternalServerError().json(ResponseStructureError {
                    success: false,
                    code: 500,
                    message: String::from(t!("file.content.read_error")),
                });
            }
            convert_gbk_to_utf8(content_vec)
        }
    };

    HttpResponse::Ok().json(ResponseStructure {
        success: true,
        code: 200,
        message: String::from("Success"),
        data: Some(
            serde_json::json!({"content":sm4_encrypt_file(content.as_str()),"ext":extension}),
        ),
    })
}

#[derive(Serialize)]
struct FileInfo {
    name: String,
    permissions: Option<u32>,
    size: Option<u64>,
    modified_time: Option<u64>,
}
#[derive(Serialize)]
struct DirInfo {
    name: String,
    permissions: Option<u32>,
    modified_time: Option<u64>,
}
#[derive(Serialize)]
struct FilesAndDirsInfo {
    path: String,
    files: Vec<FileInfo>,
    dirs: Vec<DirInfo>,
}

fn get_files_and_dirs_list(dir_path: &str) -> FilesAndDirsInfo {
    let mut files_info = Vec::new();
    let mut dirs_info = Vec::new();
    let mut p: String = String::new();

    if let Ok(cannibalized_dir) = fs::canonicalize(dir_path) {
        let cannibalized_str = cannibalized_dir.to_str().unwrap();
        if cannibalized_str.starts_with("\\\\?\\") {
            let clean_path = if cannibalized_str.starts_with("\\\\?\\UNC\\") {
                &cannibalized_str[9..]
            } else {
                &cannibalized_str[4..]
            };

            // 打印有效路径
            p = clean_path.to_string();
        } else {
            p = cannibalized_dir.to_str().unwrap().to_string();
        }
    }
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        let metadata = entry.metadata().unwrap();
                        if metadata.is_file() {
                            let info = FileInfo {
                                name: file_name_str.to_string(),
                                permissions: None,
                                size: Some(metadata.len()),
                                modified_time: Some(
                                    metadata
                                        .modified()
                                        .unwrap()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs() as u64,
                                ),
                            };

                            files_info.push(info);
                        } else if metadata.is_dir() {
                            let info = DirInfo {
                                name: file_name_str.to_string(),
                                permissions: None,
                                modified_time: Some(
                                    metadata
                                        .modified()
                                        .unwrap()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs() as u64,
                                ),
                            };

                            dirs_info.push(info);
                        }
                    }
                }
            }
        }
    }
    FilesAndDirsInfo {
        path: p,
        files: files_info,
        dirs: dirs_info,
    }
}

fn convert_gbk_to_utf8(content: Vec<u8>) -> String {
    // 将 GBK 编码的字节内容转换为字符串
    let (decoded, _, _) = GBK.decode(&content);
    // 将解码后的字符串重新编码为 UTF-8
    let (encoded, _, _) = UTF_8.encode(&decoded);
    // 返回 UTF-8 编码的字符串
    String::from_utf8_lossy(encoded.as_ref()).to_string()
}
