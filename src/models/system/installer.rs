/*
 * @Descripttion: Native Rust Installer for System Software
 * @version: 
 * @Author: Wynters
 * @Date: 2025-12-15 12:35:29
 * @LastEditTime: 2025-12-15 13:28:48
 * @FilePath: \RustPanel\src\models\system\installer.rs
 */
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::models::system::installers::{
    nginx::NginxInstaller,
    mysql::MysqlInstaller,
    pgsql::PgsqlInstaller,
    redis::RedisInstaller,
    docker::DockerInstaller,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SoftwareType {
    Nginx,
    Mysql,
    Pgsql,
    Redis,
    Docker,
}

impl fmt::Display for SoftwareType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoftwareType::Nginx => write!(f, "nginx"),
            SoftwareType::Mysql => write!(f, "mysql"),
            SoftwareType::Pgsql => write!(f, "pgsql"),
            SoftwareType::Redis => write!(f, "redis"),
            SoftwareType::Docker => write!(f, "docker"),
        }
    }
}

pub async fn install(software: SoftwareType, version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    match software {
        SoftwareType::Nginx => NginxInstaller::install(version).await,
        SoftwareType::Mysql => MysqlInstaller::install(version).await,
        SoftwareType::Pgsql => PgsqlInstaller::install(version).await,
        SoftwareType::Redis => RedisInstaller::install(version).await,
        SoftwareType::Docker => DockerInstaller::install(version).await,
    }
}

pub async fn switch_version(software: SoftwareType, version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    // For now, switch version is essentially a reinstall or specific upgrade logic.
    // We delegate to install for simplicity, assuming the installer handles overwrites/upgrades.
    install(software, version).await
}

pub async fn uninstall(software: SoftwareType) -> Result<(), Box<dyn Error + Send + Sync>> {
    match software {
        SoftwareType::Nginx => NginxInstaller::uninstall().await,
        SoftwareType::Mysql => MysqlInstaller::uninstall().await,
        SoftwareType::Pgsql => PgsqlInstaller::uninstall().await,
        SoftwareType::Redis => RedisInstaller::uninstall().await,
        SoftwareType::Docker => DockerInstaller::uninstall().await,
    }
}
