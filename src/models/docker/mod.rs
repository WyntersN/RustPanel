/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-27 17:51:07
 * @LastEditTime: 2024-06-02 17:57:25
 * @FilePath: \RustPanel\src\models\docker\mod.rs
 */
pub mod container;
pub mod image;
pub mod network;
use bollard::Docker;
use std::{error::Error, fmt};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use crate::service::global::OS_TYPE;

#[derive(Debug)]
pub struct DockerError {
    pub message: String,
}

impl fmt::Display for DockerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for DockerError {}
pub fn docker() -> Result<bollard::Docker, Box<dyn Error>> {
    match Docker::connect_with_socket_defaults() {
        Ok(res) => return Ok(res),
        Err(e) => {
            return Err(Box::new(DockerError {
                message: e.to_string(),
            }))
        }
    };
}

pub fn install() -> Result<(), Box<dyn Error>> {
    //判断 OS_TYPE 操作系统是Linux 还是 Windows 还是 Mac
    match OS_TYPE.as_str() {
        "linux" => {
            // 假设你的 shell 脚本文件名为 script.sh
            let script_path = "./install/docker.sh --source mirrors.aliyun.com/docker-ce --source-registry registry.cn-hangzhou.aliyuncs.com --install-latested true  --ignore-backup-tips";

            // 启动脚本进程，将输出和错误重定向到 Rust 程序
            let mut child = Command::new("sh")
                .arg(script_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            // 获取子进程输出的句柄
            let stdout = child.stdout.take().expect("Failed to get stdout");
            let stderr = child.stderr.take().expect("Failed to get stderr");

            // 创建用于读取输出的缓冲读取器
            let stdout_reader = BufReader::new(stdout);
            let stderr_reader = BufReader::new(stderr);

            // 读取并实时输出 stdout
            println!("Output from the script:");
            for line in stdout_reader.lines() {
                println!("{}", line?); // 打印每一行输出
            }

            // 读取并实时输出 stderr
            println!("Errors from the script:");
            for line in stderr_reader.lines() {
                eprintln!("{}", line?); // 打印每一行错误
            }

            // 等待脚本执行完成
            let status = child.wait()?;

            // 检查脚本是否成功执行
            if status.success() {
                println!("Script executed successfully!");
            } else {
                println!("Script failed to execute.");
            }

            Ok(())
        }
        "windows" => {
            // Windows 操作系统
            println!("Windows 操作系统");
            return Ok(());
        }
        "macos" => {
            // Mac 操作系统
            println!("Mac 操作系统");
            return Ok(());
        }
        _ => {
            // 其他操作系统
            println!("其他操作系统");
            return Ok(());
        }
    }
}


