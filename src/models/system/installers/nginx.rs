use std::error::Error;
use std::path::Path;
use tokio::process::Command;
use tokio::fs;
use std::process::Stdio;
use super::manager::{OsManager, PackageManager};

pub struct NginxInstaller;

impl NginxInstaller {
    pub async fn install(version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Starting Nginx {} installation...", version);

        // 1. Check and Install System Dependencies
        Self::install_system_dependencies().await?;

        // 2. Create 'www' user if not exists
        OsManager::create_user("www", "/sbin/nologin").await?;

        // 3. Download Source
        let download_url = format!("http://nginx.org/download/nginx-{}.tar.gz", version);
        let temp_dir = std::env::temp_dir().join("rustpanel_nginx");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).await?;
        }
        fs::create_dir_all(&temp_dir).await?;

        let tarball_path = temp_dir.join(format!("nginx-{}.tar.gz", version));
        
        OsManager::download_file(&download_url, &tarball_path).await?;

        // 4. Extract
        OsManager::extract_tarball(&tarball_path, &temp_dir).await?;
        let source_dir = temp_dir.join(format!("nginx-{}", version));

        // 5. Configure
        println!("Configuring...");
        let current_dir = std::env::current_dir()?;
        let install_path_buf = current_dir.join("server").join("nginx");
        let install_path = install_path_buf.as_path();
        
        if !install_path.exists() {
            fs::create_dir_all(install_path).await?;
        }

        let configure_args = vec![
            format!("--prefix={}", install_path.display()),
            "--user=www".to_string(),
            "--group=www".to_string(),
            "--with-http_stub_status_module".to_string(),
            "--with-http_ssl_module".to_string(),
            "--with-http_v2_module".to_string(),
            "--with-http_gzip_static_module".to_string(),
            "--with-stream".to_string(),
            "--with-stream_ssl_module".to_string(),
        ];

        let status = Command::new("./configure")
            .current_dir(&source_dir)
            .args(&configure_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("Nginx configure failed".into());
        }

        // 6. Make & Make Install
        println!("Compiling (make)...");
        let status = Command::new("make")
            .current_dir(&source_dir)
            .arg("-j").arg(num_cpus::get().to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("Nginx make failed".into());
        }

        println!("Installing (make install)...");
        let status = Command::new("make")
            .current_dir(&source_dir)
            .arg("install")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("Nginx make install failed".into());
        }

        // 7. Setup Service
        Self::setup_systemd_service(install_path).await?;

        // 8. Clean up
        let _ = fs::remove_dir_all(&temp_dir).await;

        println!("Nginx installed successfully!");
        Ok(())
    }

    pub async fn uninstall() -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Uninstalling Nginx...");
        // Stop service
        let _ = Command::new("systemctl").arg("stop").arg("nginx").status().await;
        let _ = Command::new("systemctl").arg("disable").arg("nginx").status().await;
        
        // Remove service file
        let service_file = Path::new("/etc/systemd/system/nginx.service");
        if service_file.exists() {
            fs::remove_file(service_file).await?;
            let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        }

        // Remove files
        let current_dir = std::env::current_dir()?;
        let install_path_buf = current_dir.join("server").join("nginx");
        let install_path = install_path_buf.as_path();
        
        if install_path.exists() {
            fs::remove_dir_all(install_path).await?;
        }

        println!("Nginx uninstalled.");
        Ok(())
    }

    async fn install_system_dependencies() -> Result<(), Box<dyn Error + Send + Sync>> {
        let pm = OsManager::detect_package_manager();
        let pkgs = match pm {
            PackageManager::Yum => vec![
                "gcc", "gcc-c++", "make", "zlib-devel", "pcre-devel", "openssl-devel",
                "libtool", "automake", "curl", "curl-devel"
            ],
            PackageManager::Apt => vec![
                "build-essential", "libpcre3", "libpcre3-dev", "zlib1g", "zlib1g-dev",
                "libssl-dev", "curl", "libcurl4-openssl-dev"
            ],
            PackageManager::Unknown => return Err("Unsupported OS for Nginx compilation".into()),
        };

        OsManager::install_dependencies(&pkgs).await
    }

    async fn setup_systemd_service(install_path: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service_content = format!(r#"
[Unit]
Description=The NGINX HTTP and reverse proxy server
After=syslog.target network-online.target remote-fs.target nss-lookup.target
Wants=network-online.target

[Service]
Type=forking
PIDFile={}/logs/nginx.pid
ExecStartPre={}/sbin/nginx -t
ExecStart={}/sbin/nginx
ExecReload={}/sbin/nginx -s reload
ExecStop=/bin/kill -s QUIT $MAINPID
PrivateTmp=true

[Install]
WantedBy=multi-user.target
"#, install_path.display(), install_path.display(), install_path.display(), install_path.display());

        fs::write("/etc/systemd/system/nginx.service", service_content).await?;
        
        let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        let _ = Command::new("systemctl").arg("enable").arg("nginx").status().await;
        
        Ok(())
    }
}
