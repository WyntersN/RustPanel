/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-07-11 04:20:17
 * @LastEditTime: 2024-07-11 06:34:29
 * @FilePath: \RustPanel\src\models\docker\image.rs
 */
use bollard::image::ListImagesOptions;
use super::docker;

pub async fn list(){

    if let Ok(client) = docker() {
        if let Ok(images) = client.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await {

            for image in images {
                println!("=============->{:?}", image);
            }
        }
    }

}