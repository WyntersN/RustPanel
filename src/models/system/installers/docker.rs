use std::error::Error;
use tokio::process::Command;
use tokio::fs;
use super::manager::{OsManager, PackageManager};

pub struct DockerInstaller;

impl DockerInstaller {
    pub async fn install(_version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Starting Docker installation...");
        // Note: Docker version argument is often ignored in standard repo installs unless specifically requested.
        // For simplicity, we install the latest stable version from official repos.

        let pm = OsManager::detect_package_manager();
        match pm {
            PackageManager::Yum => Self::install_yum().await?,
            PackageManager::Apt => Self::install_apt().await?,
            PackageManager::Unknown => return Err("Unsupported OS for Docker installation".into()),
        }

        // Enable and Start Docker
        println!("Starting Docker service...");
        let _ = Command::new("systemctl").arg("enable").arg("docker").status().await;
        let status = Command::new("systemctl").arg("start").arg("docker").status().await?;
        
        if !status.success() {
            return Err("Failed to start Docker service".into());
        }

        println!("Docker installed successfully!");
        Ok(())
    }

    pub async fn uninstall() -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Uninstalling Docker...");
        
        let pm = OsManager::detect_package_manager();
        
        // Stop service
        let _ = Command::new("systemctl").arg("stop").arg("docker").status().await;
        let _ = Command::new("systemctl").arg("disable").arg("docker").status().await;

        match pm {
            PackageManager::Yum => {
                let pkgs = vec![
                    "docker-ce", "docker-ce-cli", "containerd.io", "docker-buildx-plugin", "docker-compose-plugin",
                    "docker", "docker-client", "docker-client-latest", "docker-common", "docker-latest", "docker-latest-logrotate", "docker-logrotate", "docker-engine"
                ];
                Command::new("yum").arg("remove").arg("-y").args(&pkgs).status().await?;
            }
            PackageManager::Apt => {
                let pkgs = vec![
                    "docker-ce", "docker-ce-cli", "containerd.io", "docker-buildx-plugin", "docker-compose-plugin",
                    "docker.io", "docker-doc", "docker-compose", "podman-docker", "containerd", "runc"
                ];
                Command::new("apt-get").arg("purge").arg("-y").args(&pkgs).status().await?;
            }
            PackageManager::Unknown => return Err("Unsupported OS".into()),
        }

        // Cleanup directories
        let paths = vec!["/var/lib/docker", "/var/lib/containerd", "/etc/docker"];
        for path in paths {
            let p = std::path::Path::new(path);
            if p.exists() {
                let _ = fs::remove_dir_all(p).await;
            }
        }

        println!("Docker uninstalled.");
        Ok(())
    }

    async fn install_yum() -> Result<(), Box<dyn Error + Send + Sync>> {
        // 1. Remove old versions
        let old_pkgs = vec![
            "docker", "docker-client", "docker-client-latest", "docker-common", "docker-latest", "docker-latest-logrotate", "docker-logrotate", "docker-engine"
        ];
        let _ = Command::new("yum").arg("remove").arg("-y").args(&old_pkgs).status().await;

        // 2. Install utils
        OsManager::install_dependencies(&["yum-utils"]).await?;

        // 3. Add Repo
        println!("Adding Docker repo...");
        let status = Command::new("yum-config-manager")
            .arg("--add-repo")
            .arg("https://download.docker.com/linux/centos/docker-ce.repo")
            .status()
            .await?;
        
        if !status.success() {
            // Try to handle RedHat/Fedora if CentOS repo fails, or just proceed hoping it worked or user has repo.
            // But let's assume standard CentOS/RHEL/Alinux/etc compatibility.
             return Err("Failed to add Docker repository".into());
        }

        // 4. Install Docker
        println!("Installing Docker packages...");
        let pkgs = vec!["docker-ce", "docker-ce-cli", "containerd.io", "docker-buildx-plugin", "docker-compose-plugin"];
        OsManager::install_dependencies(&pkgs).await?;

        Ok(())
    }

    async fn install_apt() -> Result<(), Box<dyn Error + Send + Sync>> {
        // 1. Remove old versions
        let old_pkgs = vec![
            "docker.io", "docker-doc", "docker-compose", "podman-docker", "containerd", "runc"
        ];
        let _ = Command::new("apt-get").arg("remove").arg("-y").args(&old_pkgs).status().await;

        // 2. Install utils
        OsManager::install_dependencies(&["ca-certificates", "curl", "gnupg"]).await?;

        // 3. Add GPG Key
        println!("Adding Docker GPG key...");
        fs::create_dir_all("/etc/apt/keyrings").await?;
        let keyring_path = "/etc/apt/keyrings/docker.gpg";
        // Remove old if exists
        if std::path::Path::new(keyring_path).exists() {
            let _ = fs::remove_file(keyring_path).await;
        }

        // Curl pipe to gpg --dearmor
        // Since we are in Rust, let's use Command chain or just download and shell out for simplicity of pipe
        let status = Command::new("bash")
            .arg("-c")
            .arg("curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg")
            .status()
            .await?;
        
        if !status.success() {
             // Fallback for Debian if ubuntu fails? 
             // Ideally we should detect distro codename.
             // But let's try generic approach or check /etc/os-release
             // For now, assume Ubuntu/Debian compatible.
             return Err("Failed to add Docker GPG key".into());
        }
        let _ = Command::new("chmod").arg("a+r").arg(keyring_path).status().await;

        // 4. Add Repo
        println!("Adding Docker repository...");
        // Get codename
        let output = Command::new("lsb_release").arg("-cs").output().await;
        let codename = if let Ok(out) = output {
             String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else {
            // Fallback: cat /etc/os-release
            "jammy".to_string() // unsafe assumption, but standard on many. Ideally parse /etc/os-release.
        };
        
        // Better codename detection
        let codename = Self::get_distro_codename().await.unwrap_or("jammy".to_string());

        // Determine if ubuntu or debian
        let os_release = fs::read_to_string("/etc/os-release").await.unwrap_or_default().to_lowercase();
        let repo_url = if os_release.contains("debian") {
            "https://download.docker.com/linux/debian"
        } else {
            "https://download.docker.com/linux/ubuntu"
        };

        let repo_line = format!(
            "deb [arch=\"amd64\" signed-by={}] {} {} stable",
            keyring_path, repo_url, codename
        );
        
        fs::write("/etc/apt/sources.list.d/docker.list", repo_line).await?;

        // 5. Update and Install
        println!("Updating apt cache...");
        let _ = Command::new("apt-get").arg("update").status().await;

        println!("Installing Docker packages...");
        let pkgs = vec!["docker-ce", "docker-ce-cli", "containerd.io", "docker-buildx-plugin", "docker-compose-plugin"];
        OsManager::install_dependencies(&pkgs).await?;

        Ok(())
    }

    async fn get_distro_codename() -> Option<String> {
        // Try lsb_release
        if let Ok(output) = Command::new("lsb_release").arg("-cs").output().await {
            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        
        // Try parsing /etc/os-release
        if let Ok(content) = fs::read_to_string("/etc/os-release").await {
            for line in content.lines() {
                if line.starts_with("VERSION_CODENAME=") {
                    return Some(line.trim_start_matches("VERSION_CODENAME=").trim_matches('"').to_string());
                }
            }
            // Fallback for systems without VERSION_CODENAME (like older CentOS, though we are in apt block)
            // Debian usually has VERSION_CODENAME
        }
        None
    }
}
