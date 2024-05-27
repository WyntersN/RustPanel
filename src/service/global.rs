/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-08 19:57:53
 * @LastEditTime: 2024-05-27 16:24:49
 * @FilePath: \RustPanel\src\service\global.rs
 */
use lazy_static::*; 
use crate::service::conf::Config; 

pub const USER_PASSWORD_KEY:&str = "4e425cc46f42a4e01ee54a3a1e7db645e629621d";
pub const SESSION_KEY:&[u8; 64] = b"749d93ea30bf13e0b4a07d8f8fe5aa488b6c83fc886a266dd2c80590026f7b49";

lazy_static! {
    pub static ref CONF: Config = Config::load_from_file();
} 

lazy_static! {
    pub static ref OS_INFO : serde_json::Value = serde_json::json!({
        "name":sysinfo::System::name(),
        "kernel_version":sysinfo::System::kernel_version(),
        "os_version":sysinfo::System::os_version(),
        "host_name":sysinfo::System::host_name(),
        "boot_time":sysinfo::System::boot_time(),
        "long_os_version":sysinfo::System::long_os_version(),
        "architecture":os_info::get().architecture(),
        "os_type":os_info::get().os_type(),
    });
} 
