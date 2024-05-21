/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-08 14:47:19
 * @LastEditTime: 2024-05-08 16:32:47
 * @FilePath: \rust_panel\src\common\sys.rs
 */

use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::net::IpAddr;
pub fn restart() {

     let project_name = match extract_project_name(env::current_exe().expect("Failed to get current executable path")) {
        Some(name) => name,
        None => {
            println!("Project root directory not found.");
            return;
        }
    };

    match sys_info::os_type().unwrap().as_str() {
        "Linux" => {
              Command::new(project_name+r"\panel")
             .spawn()
             .expect("restart...Failed to restart the process");
        }
        "Windows" => {
            Command::new(project_name+r"\panel.exe")
                .spawn()
                .expect("restart...Failed to restart the process");
        }
        _ => {
            format!("Unsupported OS: {}", sys_info::os_type().unwrap());
        }
    }
  
}

pub fn get_all_ip_addresses() -> Result<Vec<IpAddr>, String> {
   
    let interfaces = match  get_if_addrs::get_if_addrs() {
        Ok(interfaces) => interfaces,
        Err(e) => return Err(format!("Failed to get network interfaces: {}", e)),
    };

    let mut ip_addresses = Vec::new();

    for interface in interfaces {
        ip_addresses.push(interface.ip());
    }

    Ok(ip_addresses)
}


fn extract_project_name(path: PathBuf) -> Option<String> {
    let root_dir_name = "rust_panel";
    let mut current_path = path;
    let mut project_root = None;

    while let Some(component) = current_path.file_name() {
        if component == root_dir_name {
            project_root = Some(current_path.to_string_lossy().into_owned());
            break;
        }

        current_path = match current_path.parent() {
            Some(parent) => parent.to_path_buf(),
            None => break,
        };
    }

    project_root
}
