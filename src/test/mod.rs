/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 17:41:48
 * @LastEditTime: 2024-10-26 04:57:18
 * @FilePath: \RustPanel\src\test\mod.rs
 */

use crate::models::docker;
use crate::service::db::DBPool;
use bollard::errors::Error;
use bollard::Docker;
use bollard::{
    container::{Config, CreateContainerOptions},
    image::ListImagesOptions,
    network::ListNetworksOptions,
    secret::{HostConfig, RestartPolicy, RestartPolicyNameEnum},
};
use lazy_static::lazy_static;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use std::default::Default;
use std::process::Stdio;
use std::sync::Mutex;
lazy_static! {
    pub static ref DOCKER: Mutex<Option<Docker>> =
        Mutex::new(Docker::connect_with_socket_defaults().ok());
}
pub async fn demo(_: &DBPool)-> Result<(), Box<dyn std::error::Error>> {
    //Test
    // 执行的 shell 命令
    let script_path = "./install/lib.sh";

    // 创建 Command 对象
    let mut cmd = Command::new("sh");
    cmd.arg(script_path);

    // 配置标准输出为管道模式，以便实时获取输出内容
    cmd.stdout(Stdio::piped());

    // 执行命令
    let mut child = cmd.spawn()?;
    
    // 读取标准输出的内容
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout).lines();
        tokio::pin!(reader);
        while let Some(line) = reader.next_line().await? {
            println!("{}", line); // 输出每一行的内容
        }
    }

    // 等待命令执行完毕，并获取执行状态
    let status = child.wait().await?;
    if status.success() {
        println!("Shell 脚本执行成功");
    } else {
        println!("Shell 脚本执行失败: {:?}", status.code());
    }

    Ok(())
    // let docker = DOCKER.lock().unwrap();

    // let _ = docker::install();

    // docker::image::list().await;

    // println!("->{:?}", &docker.version().await.unwrap().version.unwrap());

    // let networks = &docker
    //     .list_networks(Some(ListNetworksOptions::<String> {
    //         ..Default::default()
    //     }))
    //     .await
    //     .unwrap();


    // for network in networks {
    //     println!("-> {:?}", network.id.as_deref().unwrap_or("No ID"));
    // }

    // let alpine_config = Config {
    //     image: Some("alpine:3.19.1"),
    //     tty: Some(true),
    //     attach_stdin: Some(true),
    //     attach_stdout: Some(true),
    //     attach_stderr: Some(true),
    //     open_stdin: Some(true),
    //     host_config: Some(HostConfig {
    //         // auto_remove: Some(true),
    //         restart_policy: Some(RestartPolicy {
    //             name: Some(RestartPolicyNameEnum::ALWAYS),
    //             //maximum_retry_count: Some(10),
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // };

    // let id = match docker
    //     .create_container(
    //         Some(CreateContainerOptions {
    //             name: "test-container-3.19.1",
    //             ..Default::default()
    //         }),
    //         alpine_config,
    //     )
    //     .await
    // {
    //     Ok(res) => res.id,
    //     Err(Error::DockerResponseServerError {
    //         status_code,
    //         message,
    //     }) => {
    //         println!(
    //             "Docker responded with status code {}: {}",
    //             status_code, message
    //         );
    //         return;
    //     }
    //     Err(e) => {
    //         println!("Other error: {:?}", e);
    //         return;
    //     }
    // };

    // match docker.start_container::<String>(&id, None).await {
    //     Ok(_) => println!("-> {:?}", "Started"),
    //     Err(e) => println!("-> {:?}", e),
    // }

    // let images = &docker
    //     .list_images(Some(ListImagesOptions::<String> {
    //         all: true,
    //         ..Default::default()
    //     }))
    //     .await
    //     .unwrap();

    // for image in images {
    //     println!("-> {:?}", image.repo_tags[0]);
    // }



   

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
