/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 17:41:48
 * @LastEditTime: 2024-05-19 20:50:44
 * @FilePath: \RustPanel\src\test\mod.rs
 */


use crate::{common::sys::get_all_ip_addresses, server::db::DBPool};

pub fn demo(_: &DBPool) {




    //获取CPU信息
    let info = os_info::get();
    println!("OS information: {info}");
    // Print information separately:
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    println!("Bitness: {}", info.bitness());

    println!("{}", sys_info::os_type().unwrap());
    println!("{}", sys_info::os_release().unwrap());
    println!("{}", sys_info::cpu_num().unwrap());
    println!("{}", sys_info::cpu_speed().unwrap());

    match get_all_ip_addresses() {
        Ok(ip_addresses) => {
            println!("All IP addresses:");
            for ip in ip_addresses {
                println!("{}", ip);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    match sys_info::mem_info() {
        Ok(info) => {
            println!("{:?}", info);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    //let mut contents = String::new();
    // match File::open("./config/conf.yaml") {
    //     Ok(mut file) => {
    //         match file.read_to_string(&mut contents) {
    //             Ok(_) => println!("File contents: {}", contents),
    //             Err(err) => eprintln!("Error reading file: {}", err),
    //         }
    //     }
    //     Err(err) => eprintln!("Error opening file: {}", err),
    // }
}
