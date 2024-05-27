/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-14 15:00:17
 * @LastEditTime: 2024-05-27 16:24:36
 * @FilePath: \RustPanel\src\api\v1\os.rs
 */

use actix_web::HttpResponse;
use sysinfo::{Disks, Networks, System};

use crate::{api::{auth::AuthUser, ResponseStructure}, service::global::OS_INFO};
use serde_json::json;
pub async fn os_info(_: AuthUser) -> HttpResponse {
    let mut sys = System::new_all();


    let timestamp = chrono::Local::now().timestamp();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); //由于要兼容VM虚拟机，所以需要等待一下才能获取到数据
    sys.refresh_cpu();
    let mut cpu_info: Vec<f32> = Vec::new(); 
    for cpu in sys.cpus() {
        cpu_info.push(cpu.cpu_usage());
    }

    let mut disk_info: Vec<serde_json::Value> = Vec::new(); 
    for disk in &Disks::new_with_refreshed_list() {
        let mount_point = disk.mount_point().to_str();
        if mount_point.is_none() || mount_point.unwrap().to_string().contains("docker") {
            continue;
        } 

        disk_info.push(json!({
            "mount_point": mount_point,
            "total": disk.total_space()/1048576,
            "available": disk.available_space()/1048576,
        }))
    }

    let mut network_info: Vec<serde_json::Value> = Vec::new(); 
    for (interface_name, data) in &Networks::new_with_refreshed_list() {
        
        if interface_name.to_string().contains("NPCAP") {
            continue;
        }
        network_info.push(json!({
            "name": interface_name,
            "total_received": data.total_received()/1024,
            "total_transmitted": data.total_transmitted()/1024,
            "received": data.received(),
            "transmitted": data.transmitted(),
        }))
    }

    HttpResponse::Ok().json(Some(ResponseStructure {
        success: true,
        code: 200,
        message: String::from("success"),
        data: Some(json!({
            "updated_at":timestamp,
            "os":OS_INFO.clone(),
            //"fz":calculate_load_percentage(cpu_info.len() as u8).unwrap_or(0.0),
            "memory":{
                "swap_total":sys.total_swap()/1048576,
                "swap_used":sys.used_swap()/1048576,
                "total":sys.total_memory()/1048576,
                "used":sys.used_memory()/1048576,
            },
            "cpu":cpu_info,
            "disk":disk_info,
            "network":network_info
        })),
    }))
}


// fn calculate_load_percentage(cpu_num:u8) -> io::Result<f64> {
//     let path = std::path::Path::new("./runtime/loadavg");
//     let contents = fs::read_to_string(path)?;

//     // 分割字符串并获取1分钟负载和CPU核心数
//     let parts: Vec<&str> = contents.trim().split_whitespace().collect();
//     if parts.len() < 1 {
//         return Err(io::Error::new(io::ErrorKind::InvalidData, "Not enough data in /proc/loadavg"));
//     }
//     let load_1_min: f64 = parts[0].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid load average"))?;


//     // 计算负载百分比
//     let load_percentage = load_1_min / cpu_num as f64 * 100.0;

//     Ok(load_percentage)
// }