/*
 * @Descripttion:
 * @version:
 * @Author: Wynters
 * @Date: 2024-05-07 17:41:48
 * @LastEditTime: 2024-05-29 20:46:58
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
use std::default::Default;
use std::sync::Mutex;
lazy_static! {
    pub static ref DOCKER: Mutex<Docker> =
        Mutex::new(Docker::connect_with_socket_defaults().unwrap());
}
pub async fn demo(_: &DBPool) {
    let docker = DOCKER.lock().unwrap();

    let _ = docker::install();

    docker::image::list().await;

    println!("->{:?}", &docker.version().await.unwrap().version.unwrap());

    let networks = &docker
        .list_networks(Some(ListNetworksOptions::<String> {
            ..Default::default()
        }))
        .await
        .unwrap();

    println!("---------------------------------------");

    for network in networks {
        println!("-> {:?}", network.id.as_deref().unwrap_or("No ID"));
    }

    let alpine_config = Config {
        image: Some("alpine:3.19.1"),
        tty: Some(true),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        open_stdin: Some(true),
        host_config: Some(HostConfig {
            // auto_remove: Some(true),
            restart_policy: Some(RestartPolicy {
                name: Some(RestartPolicyNameEnum::ALWAYS),
                //maximum_retry_count: Some(10),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let id = match docker
        .create_container(
            Some(CreateContainerOptions {
                name: "test-container-3.19.1",
                ..Default::default()
            }),
            alpine_config,
        )
        .await
    {
        Ok(res) => res.id,
        Err(Error::DockerResponseServerError {
            status_code,
            message,
        }) => {
            println!(
                "Docker responded with status code {}: {}",
                status_code, message
            );
            return;
        }
        Err(e) => {
            println!("Other error: {:?}", e);
            return;
        }
    };

    match docker.start_container::<String>(&id, None).await {
        Ok(_) => println!("-> {:?}", "Started"),
        Err(e) => println!("-> {:?}", e),
    }

    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();
    println!("---------------------------------------");

    for image in images {
        println!("-> {:?}", image.repo_tags[0]);
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
