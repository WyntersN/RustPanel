use bollard::image::ListImagesOptions;
use super::docker;

pub async fn list(){

    let images = docker().unwrap().list_images(Some(ListImagesOptions::<String> {
        all: true,
        ..Default::default()
    })).await.unwrap();

    println!("-------------------------------------");

    for image in images {
        println!("=============->{:?}", image);
    }

}