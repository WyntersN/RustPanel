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

    let images = docker().unwrap().list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    for image in images {
        println!("=============->{:?}", image);
    }

}