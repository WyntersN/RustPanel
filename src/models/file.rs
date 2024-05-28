/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-28 20:10:12
 * @LastEditTime: 2024-05-28 20:55:45
 * @FilePath: \RustPanel\src\models\file.rs
 */

use std::{error::Error, fmt, fs};

use encoding_rs::{GBK, UTF_8};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::common::fun::{sm4_decrypt_file, sm4_encrypt_file};
use crate::service::global::CONF;

#[derive(Deserialize)]
pub struct SaveData {
    pub path: String,
    pub content: String,
}
// 自定义错误类型
#[derive(Debug)]
pub struct FileError {
    pub message: String,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FileError {}

pub fn save(data: SaveData) -> Result<bool, Box<dyn Error>> {
    println!("-------------------------{}", data.path);

    // 保存文件
    match fs::write(&data.path, sm4_decrypt_file(data.content.to_string())) {
        Ok(_) => Ok(true),
        Err(e) => Err(Box::new(FileError {
            message: String::from(t!("file.save.write_error", error => e.to_string())),
        })),
    }
}

pub fn content(path: String) -> Result<(String, String), Box<dyn Error>> {
    let path = Path::new(&path);
    // 获取文件元数据
    let metadata = fs::metadata(path).map_err(|_| {
        // 如果无法获取元数据，返回错误
        return Box::new(FileError {
            message: String::from(t!("file.content.info_error")),
        });
    });

    // 先定义一个外部变量，用于存储文件的扩展名
    let mut extension = String::new();

    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
        extension = ext.to_string();
        if extension == "exe" || extension == "dll" || extension == "so" || extension == "zip" {
            return Err(Box::new(FileError {
                message: String::from(t!("file.content.file_type_error")),
            }));
        }
    }

    // 检查文件大小是否超过 N MB
    if metadata.unwrap().len() as f64 / 1048576.0 > CONF.app.max_file_size {
        // 如果文件大小超过10MB，返回错误
        return Err(Box::new(FileError {
            message: String::from(t!("file.content.file_max_size_error",size => CONF.app.max_file_size)),
        }));
    }

    // 打开文件
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Err(Box::new(FileError {
                message: String::from(t!("file.content.open_error")),
            }));
        }
    };

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            // 读取文件内容到一个字节数组中
            let mut content_vec = Vec::new();
            if let Err(_) = file.read_to_end(&mut content_vec) {
                return Err(Box::new(FileError {
                    message: String::from(t!("file.content.read_error")),
                }));
            }
            convert_gbk_to_utf8(content_vec)
        }
    };

    Ok((sm4_encrypt_file(content.as_str()),extension))
}

fn convert_gbk_to_utf8(content: Vec<u8>) -> String {
    // 将 GBK 编码的字节内容转换为字符串
    let (decoded, _, _) = GBK.decode(&content);
    // 将解码后的字符串重新编码为 UTF-8
    let (encoded, _, _) = UTF_8.encode(&decoded);
    // 返回 UTF-8 编码的字符串
    String::from_utf8_lossy(encoded.as_ref()).to_string()
}


#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub permissions: Option<u32>,
    pub size: Option<u64>,
    pub modified_time: Option<u64>,
}
#[derive(Serialize)]
pub struct DirInfo {
    pub name: String,
    pub permissions: Option<u32>,
    pub modified_time: Option<u64>,
}
#[derive(Serialize)]
pub struct FilesAndDirsInfo {
    pub path: String,
    pub files: Vec<FileInfo>,
    pub dirs: Vec<DirInfo>,
}

pub fn get_files_and_dirs_list(dir_path: &str) -> FilesAndDirsInfo {
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
