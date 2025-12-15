use std::error::Error;
use tokio::process::Command;
use os_info::{Type, Info};

#[derive(Debug, Clone, PartialEq)]
pub enum PackageManager {
    Yum,
    Apt,
    Unknown,
}

pub struct OsManager;

impl OsManager {
    pub fn get_os_info() -> Info {
        os_info::get()
    }

    pub fn detect_package_manager() -> PackageManager {
        let info = Self::get_os_info();
        match info.os_type() {
            Type::CentOS | Type::Redhat | Type::Fedora | Type::Amazon => PackageManager::Yum,
            Type::Ubuntu | Type::Debian | Type::Pop | Type::Mint => PackageManager::Apt,
            _ => {
                // Fallback check
                if std::path::Path::new("/usr/bin/yum").exists() || std::path::Path::new("/usr/bin/dnf").exists() {
                    PackageManager::Yum
                } else if std::path::Path::new("/usr/bin/apt-get").exists() {
                    PackageManager::Apt
                } else {
                    PackageManager::Unknown
                }
            }
        }
    }

    pub async fn install_dependencies(packages: &[&str]) -> Result<(), Box<dyn Error + Send + Sync>> {
        let pm = Self::detect_package_manager();
        let (cmd_name, install_args) = match pm {
            PackageManager::Yum => ("yum", vec!["install", "-y"]),
            PackageManager::Apt => {
                // Update apt cache first? optional but recommended
                let _ = Command::new("apt-get").arg("update").status().await;
                ("apt-get", vec!["install", "-y"])
            },
            PackageManager::Unknown => return Err("Unsupported OS or Package Manager".into()),
        };

        println!("Installing dependencies using {}: {:?}", cmd_name, packages);
        
        let status = Command::new(cmd_name)
            .args(&install_args)
            .args(packages)
            .status()
            .await?;

        if !status.success() {
            return Err(format!("Failed to install packages: {:?}", packages).into());
        }

        Ok(())
    }

    pub async fn create_user(username: &str, shell: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
         // Check if user exists
         let status = Command::new("id").arg(username).status().await;
         if status.map(|s| s.success()).unwrap_or(false) {
             return Ok(());
         }
 
         println!("Creating user '{}'...", username);
         let status = Command::new("useradd")
             .arg("-s").arg(shell)
             .arg("-M") // no home dir
             .arg(username)
             .status()
             .await?;
             
         if !status.success() {
             return Err(format!("Failed to create user {}", username).into());
         }
         Ok(())
    }

    pub async fn download_file(url: &str, path: &std::path::Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        use tokio::fs;
        println!("Downloading {} to {:?}", url, path);
        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            return Err(format!("Failed to download file: {}", response.status()).into());
        }
        let content = response.bytes().await?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(path, content).await?;
        Ok(())
    }

    pub async fn extract_tarball(tar_path: &std::path::Path, dest: &std::path::Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Extracting {:?} to {:?}", tar_path, dest);
        
        // Use system tar for broader compatibility (gzip, xz, bzip2)
        let status = Command::new("tar")
            .arg("-xf")
            .arg(tar_path)
            .arg("-C")
            .arg(dest)
            .status()
            .await?;

        if !status.success() {
            return Err(format!("Failed to extract archive: {:?}", tar_path).into());
        }
        Ok(())
    }
}
