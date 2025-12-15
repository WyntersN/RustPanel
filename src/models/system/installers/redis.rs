use std::error::Error;
use std::path::Path;
use tokio::process::Command;
use tokio::fs;
use std::process::Stdio;
use super::manager::{OsManager, PackageManager};

pub struct RedisInstaller;

impl RedisInstaller {
    pub async fn install(version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Starting Redis {} installation...", version);

        // 1. Install dependencies
        Self::install_dependencies().await?;

        // 2. Download and Extract
        let download_url = format!("http://download.redis.io/releases/redis-{}.tar.gz", version);
        let temp_dir = std::env::temp_dir().join("rustpanel_redis");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).await?;
        }
        fs::create_dir_all(&temp_dir).await?;

        let tarball_path = temp_dir.join(format!("redis-{}.tar.gz", version));
        OsManager::download_file(&download_url, &tarball_path).await?;
        OsManager::extract_tarball(&tarball_path, &temp_dir).await?;

        let source_dir = temp_dir.join(format!("redis-{}", version));

        // 3. Compile
        println!("Compiling Redis...");
        let status = Command::new("make")
            .current_dir(&source_dir)
            .arg("-j").arg(num_cpus::get().to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("Redis make failed".into());
        }

        // 4. Install
        let current_dir = std::env::current_dir()?;
        let install_path_buf = current_dir.join("server").join("redis");
        let install_path = install_path_buf.as_path();

        if !install_path.exists() {
            fs::create_dir_all(install_path).await?;
        }

        println!("Installing Redis...");
        let status = Command::new("make")
            .current_dir(&source_dir)
            .arg("install")
            .arg(format!("PREFIX={}", install_path.display()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("Redis make install failed".into());
        }

        // 5. Setup Config
        let config_src = source_dir.join("redis.conf");
        let config_dest = install_path.join("redis.conf");
        fs::copy(config_src, &config_dest).await?;

        // Update config for daemonize and bind
        let mut config_content = fs::read_to_string(&config_dest).await?;
        config_content = config_content.replace("daemonize no", "daemonize yes");
        config_content = config_content.replace("bind 127.0.0.1 -::1", "bind 127.0.0.1"); // simplify bind
        fs::write(&config_dest, config_content).await?;

        // 6. Setup Service
        Self::setup_service(install_path).await?;

        // 7. Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;

        println!("Redis installed successfully!");
        Ok(())
    }

    pub async fn uninstall() -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Uninstalling Redis...");
        let _ = Command::new("systemctl").arg("stop").arg("redis").status().await;
        let _ = Command::new("systemctl").arg("disable").arg("redis").status().await;

        let service_file = Path::new("/etc/systemd/system/redis.service");
        if service_file.exists() {
            fs::remove_file(service_file).await?;
            let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        }

        let current_dir = std::env::current_dir()?;
        let install_path_buf = current_dir.join("server").join("redis");
        let install_path = install_path_buf.as_path();

        if install_path.exists() {
            fs::remove_dir_all(install_path).await?;
        }

        println!("Redis uninstalled.");
        Ok(())
    }

    async fn install_dependencies() -> Result<(), Box<dyn Error + Send + Sync>> {
        let pm = OsManager::detect_package_manager();
        let pkgs = match pm {
            PackageManager::Yum => vec!["gcc", "gcc-c++", "make", "tcl"],
            PackageManager::Apt => vec!["build-essential", "tcl"],
            PackageManager::Unknown => return Err("Unsupported OS".into()),
        };
        OsManager::install_dependencies(&pkgs).await
    }

    async fn setup_service(install_path: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service_content = format!(r#"
[Unit]
Description=redis-server
After=network.target

[Service]
Type=forking
ExecStart={}/bin/redis-server {}/redis.conf
PrivateTmp=true

[Install]
WantedBy=multi-user.target
"#, install_path.display(), install_path.display());

        fs::write("/etc/systemd/system/redis.service", service_content).await?;
        let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        let _ = Command::new("systemctl").arg("enable").arg("redis").status().await;
        
        Ok(())
    }
}
