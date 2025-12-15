use std::error::Error;
use tokio::process::Command;
use tokio::fs;
use super::manager::{OsManager, PackageManager};

pub struct DockerInstaller;

impl DockerInstaller {
    pub async fn install(version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Starting Docker installation...");
        
        // 1. Install System Dependencies
        Self::install_system_dependencies().await?;

        // 2. Download Static Binaries
        // If version is "latest" or empty, we default to a known stable version to ensure URL validity
        // Or we could try to resolve "latest", but for static builds, explicit versions are safer.
        let docker_version = if version.is_empty() || version == "latest" {
            "27.3.1" // Hardcoded recent stable version
        } else {
            version
        };

        println!("Downloading Docker {} static binaries...", docker_version);
        let download_url = format!("https://download.docker.com/linux/static/stable/x86_64/docker-{}.tgz", docker_version);
        
        let temp_dir = std::env::temp_dir().join("rustpanel_docker");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).await?;
        }
        fs::create_dir_all(&temp_dir).await?;

        let tarball_path = temp_dir.join(format!("docker-{}.tgz", docker_version));
        OsManager::download_file(&download_url, &tarball_path).await?;

        // 3. Extract
        OsManager::extract_tarball(&tarball_path, &temp_dir).await?;
        // Structure is temp_dir/docker/{dockerd, docker, ...}

        // 4. Install to Server Directory
        let current_dir = std::env::current_dir()?;
        let install_base = current_dir.join("server").join("docker"); // e.g., /rust/RustPanel/server/docker
        let bin_dir = install_base.join("bin");
        let data_dir = install_base.join("data");
        let config_dir = install_base.join("config");

        // Clean old if exists
        if install_base.exists() {
             // Stop service first just in case
            let _ = Command::new("systemctl").arg("stop").arg("docker").status().await;
            fs::remove_dir_all(&install_base).await?;
        }

        fs::create_dir_all(&bin_dir).await?;
        fs::create_dir_all(&data_dir).await?;
        fs::create_dir_all(&config_dir).await?;

        // Move binaries
        let extracted_docker_dir = temp_dir.join("docker");
        let mut entries = fs::read_dir(&extracted_docker_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                fs::rename(&path, bin_dir.join(file_name)).await?;
            }
        }

        // 5. Configure daemon.json
        println!("Configuring Docker...");
        let daemon_json_path = config_dir.join("daemon.json");
        let daemon_config = format!(r#"
{{
  "data-root": "{}"
}}
"#, data_dir.display().to_string().replace("\\", "/")); // JSON needs forward slashes or escaped backslashes

        fs::write(&daemon_json_path, daemon_config).await?;

        // 6. Setup Service
        Self::setup_service(&bin_dir, &daemon_json_path).await?;

        // 7. Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;

        println!("Docker installed successfully to {}!", install_base.display());
        Ok(())
    }

    pub async fn uninstall() -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Uninstalling Docker...");
        
        // Stop service
        let _ = Command::new("systemctl").arg("stop").arg("docker").status().await;
        let _ = Command::new("systemctl").arg("disable").arg("docker").status().await;
        
        let service_file = std::path::Path::new("/etc/systemd/system/docker.service");
        if service_file.exists() {
            fs::remove_file(service_file).await?;
            let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        }

        // Remove files
        let current_dir = std::env::current_dir()?;
        let install_base = current_dir.join("server").join("docker");
        if install_base.exists() {
            fs::remove_dir_all(install_base).await?;
        }

        println!("Docker uninstalled.");
        Ok(())
    }

    async fn install_system_dependencies() -> Result<(), Box<dyn Error + Send + Sync>> {
        let pm = OsManager::detect_package_manager();
        let pkgs = match pm {
            PackageManager::Yum => vec![
                "iptables", "git", "xz", "procps" // Basic requirements
            ],
            PackageManager::Apt => vec![
                "iptables", "git", "xz-utils", "procps", "iproute2"
            ],
            PackageManager::Unknown => return Err("Unsupported OS".into()),
        };
        OsManager::install_dependencies(&pkgs).await
    }

    async fn setup_service(bin_dir: &std::path::Path, config_path: &std::path::Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        let dockerd_path = bin_dir.join("dockerd");
        
        // Add bin_dir to PATH in environment or just absolute path
        // We need to make sure 'containerd' and 'runc' are found.
        // Docker static bundle puts them all in same dir.
        // dockerd needs them in PATH.
        
        let service_content = format!(r#"
[Unit]
Description=Docker Application Container Engine
Documentation=https://docs.docker.com
After=network-online.target firewalld.service
Wants=network-online.target

[Service]
Type=notify
Environment="PATH={}:/usr/bin:/usr/local/bin"
ExecStart={} --config-file {}
ExecReload=/bin/kill -s HUP $MAINPID
TimeoutSec=0
RestartSec=2
Restart=always
StartLimitBurst=3
StartLimitInterval=60s
LimitNOFILE=infinity
LimitNPROC=infinity
LimitCORE=infinity
TasksMax=infinity
Delegate=yes
KillMode=process
OOMScoreAdjust=-500

[Install]
WantedBy=multi-user.target
"#, bin_dir.display(), dockerd_path.display(), config_path.display());

        fs::write("/etc/systemd/system/docker.service", service_content).await?;
        
        let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        let _ = Command::new("systemctl").arg("enable").arg("docker").status().await;
        let status = Command::new("systemctl").arg("start").arg("docker").status().await?;
        
        if !status.success() {
             return Err("Failed to start Docker service".into());
        }
        
        Ok(())
    }
}
