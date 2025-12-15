use std::error::Error;
use std::path::Path;
use tokio::process::Command;
use tokio::fs;
use super::manager::{OsManager, PackageManager};

pub struct MysqlInstaller;

impl MysqlInstaller {
    pub async fn install(version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Starting MySQL {} installation...", version);
        
        // 1. Install Dependencies (libaio is critical for MySQL)
        Self::install_dependencies().await?;

        // 2. Create mysql user
        OsManager::create_user("mysql", "/sbin/nologin").await?;

        // 3. Download Generic Binary (GLIBC)
        // URL pattern: https://downloads.mysql.com/archives/get/p/23/file/mysql-8.0.33-linux-glibc2.12-x86_64.tar.xz
        // Simplified logic: Assuming 8.0.x. For 5.7, pattern might differ.
        // For robustness, we might need a version map. For now, assume modern 8.0 versions.
        let download_url = format!("https://downloads.mysql.com/archives/get/p/23/file/mysql-{}-linux-glibc2.12-x86_64.tar.xz", version);
        
        let temp_dir = std::env::temp_dir().join("rustpanel_mysql");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).await?;
        }
        fs::create_dir_all(&temp_dir).await?;

        let tarball_path = temp_dir.join(format!("mysql-{}.tar.xz", version));
        OsManager::download_file(&download_url, &tarball_path).await?;

        // 4. Extract
        OsManager::extract_tarball(&tarball_path, &temp_dir).await?;

        // 5. Move to install path
        let install_path = Path::new("/www/server/mysql");
        if install_path.exists() {
            // Backup or fail? For now, we assume fresh install or overwrite.
            // But mv will fail if dir exists.
            let _ = fs::remove_dir_all(install_path).await;
        }
        
        // Find extracted dir
        let mut entries = fs::read_dir(&temp_dir).await?;
        let mut extracted_dir = None;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() && path.file_name().unwrap().to_string_lossy().starts_with("mysql-") {
                extracted_dir = Some(path);
                break;
            }
        }

        let source_dir = extracted_dir.ok_or("Failed to find extracted MySQL directory")?;
        fs::rename(source_dir, install_path).await?;

        // 6. Initialize Database
        println!("Initializing MySQL...");
        // mkdir data dir
        let data_dir = install_path.join("data");
        fs::create_dir_all(&data_dir).await?;
        
        // chown -R mysql:mysql
        Command::new("chown")
            .arg("-R")
            .arg("mysql:mysql")
            .arg(install_path)
            .status()
            .await?;

        // mysqld --initialize-insecure --user=mysql --basedir=... --datadir=...
        let status = Command::new(install_path.join("bin/mysqld"))
            .arg("--initialize-insecure") // No password for root initially
            .arg("--user=mysql")
            .arg(format!("--basedir={}", install_path.display()))
            .arg(format!("--datadir={}", data_dir.display()))
            .status()
            .await?;

        if !status.success() {
            return Err("MySQL initialization failed".into());
        }

        // 7. Setup my.cnf
        let my_cnf_content = format!(r#"
[client]
port = 3306
socket = /tmp/mysql.sock

[mysqld]
port = 3306
socket = /tmp/mysql.sock
basedir = {}
datadir = {}
user = mysql
bind-address = 127.0.0.1
"#, install_path.display(), data_dir.display());

        fs::write("/etc/my.cnf", my_cnf_content).await?;

        // 8. Setup Service
        Self::setup_service(install_path).await?;

        // 9. Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;

        println!("MySQL installed successfully! Root password is empty.");
        Ok(())
    }

    pub async fn uninstall() -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Uninstalling MySQL...");
        let _ = Command::new("systemctl").arg("stop").arg("mysqld").status().await;
        let _ = Command::new("systemctl").arg("disable").arg("mysqld").status().await;

        let service_file = Path::new("/etc/systemd/system/mysqld.service");
        if service_file.exists() {
            fs::remove_file(service_file).await?;
            let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        }

        let install_path = Path::new("/www/server/mysql");
        if install_path.exists() {
            fs::remove_dir_all(install_path).await?;
        }
        
        let _ = fs::remove_file("/etc/my.cnf").await;

        println!("MySQL uninstalled.");
        Ok(())
    }

    async fn install_dependencies() -> Result<(), Box<dyn Error + Send + Sync>> {
        let pm = OsManager::detect_package_manager();
        let pkgs = match pm {
            PackageManager::Yum => vec!["libaio", "ncurses-compat-libs"], // ncurses might be needed
            PackageManager::Apt => vec!["libaio1", "libncurses5"],
            PackageManager::Unknown => return Err("Unsupported OS".into()),
        };
        OsManager::install_dependencies(&pkgs).await
    }

    async fn setup_service(install_path: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service_content = format!(r#"
[Unit]
Description=MySQL Server
After=network.target

[Service]
User=mysql
Group=mysql
ExecStart={}/bin/mysqld --defaults-file=/etc/my.cnf
PrivateTmp=true

[Install]
WantedBy=multi-user.target
"#, install_path.display());

        fs::write("/etc/systemd/system/mysqld.service", service_content).await?;
        let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        let _ = Command::new("systemctl").arg("enable").arg("mysqld").status().await;
        
        Ok(())
    }
}
