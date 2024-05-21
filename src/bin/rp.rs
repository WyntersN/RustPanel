/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 16:49:06
 * @LastEditTime: 2024-05-08 16:40:12
 * @FilePath: \rust_panel\src\bin\rp.rs
 */


use rust_panel::common;
fn main() {
    match sys_info::os_type().unwrap().as_str() {
        "Linux" => {
            println!("OK");
        }
        "Windows" => {
           common::sys::restart()
    
        }
        _ => {
            format!("Unsupported OS: {}", sys_info::os_type().unwrap());
        }
    }
}
