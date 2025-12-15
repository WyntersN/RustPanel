/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 16:49:06
 * @LastEditTime: 2025-12-15 13:17:54
 * @FilePath: \RustPanel\src\bin\rp.rs
 */


use rust_panel::common;
fn main() {
    match sys_info::os_type() {
        Ok(os_type) => {
            match os_type.as_str() {
                "Linux" => {
                    println!("OK");
                }
                "Windows" => {
                   common::sys::restart()
                
                }
                _ => {
                    let _ = format!("Unsupported OS: {}", os_type);
                }
            }
        }
        Err(e) => {
            let _ = format!("Error getting OS type: {:?}", e);
        }
    }
}
