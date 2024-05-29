/*
 * @Descripttion: 
 * @version: 
 * @Author: Wynters
 * @Date: 2024-05-27 17:51:07
 * @LastEditTime: 2024-05-29 21:02:28
 * @FilePath: \RustPanel\src\models\docker\mod.rs
 */
pub mod network;
pub mod image;
pub mod container;
use bollard::Docker;
use std::{error::Error, fmt};

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
pub fn docker() -> Result<bollard::Docker,  Box<dyn Error>> {
    
   match Docker::connect_with_socket_defaults() {
        Ok(res) => return Ok(res),
        Err(e) => return Err(Box::new(DockerError {
            message: e.to_string(),
        }))
    };
}