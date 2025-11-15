/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 16:49:06
 * @LastEditTime: 2025-11-15 22:46:13
 * @FilePath: \RustPanel\src\bin\rp.rs
 */


use rust_panel::common;
fn main() {
    match match sys_info::os_type() { Ok(s) => s.as_str(), Err(_) => "" } {
        "Linux" => {
            println!("OK");
        }
        "Windows" => {
           common::sys::restart()
    
        }
        _ => {
            format!("Unsupported OS: {}", sys_info::os_type().unwrap_or_else(|_| String::from("unknown")));
        }
    }
}
