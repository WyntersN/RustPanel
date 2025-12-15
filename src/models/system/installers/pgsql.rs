use std::error::Error;
use std::path::Path;
use tokio::process::Command;
use tokio::fs;
use std::process::Stdio;
use super::manager::{OsManager, PackageManager};

pub struct PgsqlInstaller;

impl PgsqlInstaller {
    pub async fn install(version: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Starting PostgreSQL {} installation...", version);

        // 1. Install Dependencies
        Self::install_dependencies().await?;

        // 2. Create postgres user
        OsManager::create_user("postgres", "/bin/bash").await?;

        // 3. Download Source
        let download_url = format!("https://ftp.postgresql.org/pub/source/v{}/postgresql-{}.tar.gz", version, version);
        let temp_dir = std::env::temp_dir().join("rustpanel_pgsql");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).await?;
        }
        fs::create_dir_all(&temp_dir).await?;

        let tarball_path = temp_dir.join(format!("postgresql-{}.tar.gz", version));
        OsManager::download_file(&download_url, &tarball_path).await?;
        
        // 4. Extract
        OsManager::extract_tarball(&tarball_path, &temp_dir).await?;
        let source_dir = temp_dir.join(format!("postgresql-{}", version));

        // 5. Configure
        println!("Configuring PostgreSQL...");
        let install_path = Path::new("/www/server/pgsql");
        if !install_path.exists() {
            fs::create_dir_all(install_path).await?;
        }

        let status = Command::new("./configure")
            .current_dir(&source_dir)
            .arg(format!("--prefix={}", install_path.display()))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("PostgreSQL configure failed".into());
        }

        // 6. Make & Install
        println!("Compiling PostgreSQL...");
        let status = Command::new("make")
            .current_dir(&source_dir)
            .arg("-j").arg(num_cpus::get().to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("PostgreSQL make failed".into());
        }

        println!("Installing PostgreSQL...");
        let status = Command::new("make")
            .current_dir(&source_dir)
            .arg("install")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await?;

        if !status.success() {
            return Err("PostgreSQL make install failed".into());
        }

        // 7. Initialize Data Directory
        println!("Initializing PostgreSQL Data...");
        let data_dir = install_path.join("data");
        fs::create_dir_all(&data_dir).await?;
        
        // chown -R postgres:postgres
        Command::new("chown")
            .arg("-R")
            .arg("postgres:postgres")
            .arg(install_path)
            .status()
            .await?;

        // initdb -D data_dir
        // Must be run as postgres user
        let initdb_path = install_path.join("bin/initdb");
        let status = Command::new("su")
            .arg("-")
            .arg("postgres")
            .arg("-c")
            .arg(format!("{} -D {}", initdb_path.display(), data_dir.display()))
            .status()
            .await?;

        if !status.success() {
            return Err("PostgreSQL initdb failed".into());
        }

        // 8. Setup Service
        Self::setup_service(install_path, &data_dir).await?;

        // 9. Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;

        println!("PostgreSQL installed successfully!");
        Ok(())
    }

    pub async fn uninstall() -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Uninstalling PostgreSQL...");
        let _ = Command::new("systemctl").arg("stop").arg("postgresql").status().await;
        let _ = Command::new("systemctl").arg("disable").arg("postgresql").status().await;

        let service_file = Path::new("/etc/systemd/system/postgresql.service");
        if service_file.exists() {
            fs::remove_file(service_file).await?;
            let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        }

        let install_path = Path::new("/www/server/pgsql");
        if install_path.exists() {
            fs::remove_dir_all(install_path).await?;
        }

        println!("PostgreSQL uninstalled.");
        Ok(())
    }

    async fn install_dependencies() -> Result<(), Box<dyn Error + Send + Sync>> {
        let pm = OsManager::detect_package_manager();
        let pkgs = match pm {
            PackageManager::Yum => vec!["readline-devel", "zlib-devel", "gcc", "make"],
            PackageManager::Apt => vec!["libreadline-dev", "zlib1g-dev", "build-essential"],
            PackageManager::Unknown => return Err("Unsupported OS".into()),
        };
        OsManager::install_dependencies(&pkgs).await
    }

    async fn setup_service(install_path: &Path, data_dir: &Path) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service_content = format!(r#"
[Unit]
Description=PostgreSQL database server
After=network.target

[Service]
Type=forking
User=postgres
Group=postgres
Environment=PGDATA={}
ExecStart={}/bin/pg_ctl start -D ${{PGDATA}} -s -w -t 300
ExecStop={}/bin/pg_ctl stop -D ${{PGDATA}} -s -m fast
ExecReload={}/bin/pg_ctl reload -D ${{PGDATA}} -s
PrivateTmp=true

[Install]
WantedBy=multi-user.target
"#, data_dir.display(), install_path.display(), install_path.display(), install_path.display());

        fs::write("/etc/systemd/system/postgresql.service", service_content).await?;
        let _ = Command::new("systemctl").arg("daemon-reload").status().await;
        let _ = Command::new("systemctl").arg("enable").arg("postgresql").status().await;
        
        Ok(())
    }
}
