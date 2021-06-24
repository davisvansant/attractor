use crate::Attractor;

use std::path::{Path, PathBuf};

use tokio::fs::create_dir_all;
use tokio::process::Command;

enum Utility {
    AptGet,
}

async fn get_full_path(utility: Utility) -> Result<PathBuf, std::io::Error> {
    let utility = match utility {
        Utility::AptGet => String::from("apt-get"),
    };
    let mut path_buf = PathBuf::with_capacity(15);
    let command = Command::new("/usr/bin/which").arg(utility).output().await?;
    let string = String::from_utf8(command.stdout).expect("Failed to convert to String!");

    path_buf.push(string.trim());

    Ok(path_buf)
}

async fn update(path: &Path) -> Result<bool, std::io::Error> {
    let command = Command::new(path).arg("update").status().await?;

    Ok(command.success())
}

async fn install(path: &Path) -> Result<bool, std::io::Error> {
    let command = Command::new(path)
        .arg("install")
        .arg("debootstrap")
        .arg("-y")
        .status()
        .await?;

    Ok(command.success())
}

async fn filesystem() -> Result<(), std::io::Error> {
    let dir = "/var/opt/attractor/target";

    create_dir_all(dir).await?;

    Ok(())
}

impl Attractor {
    pub async fn prep() -> Result<(), std::io::Error> {
        let apt_get = crate::init::get_full_path(Utility::AptGet).await?;

        crate::init::update(&apt_get).await?;
        crate::init::install(&apt_get).await?;

        filesystem().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn get_full_path() -> Result<(), std::io::Error> {
        let test_path_buf = crate::init::get_full_path(Utility::AptGet).await?;
        assert_eq!(test_path_buf.to_str().unwrap(), "/usr/bin/apt-get");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn update_and_install() -> Result<(), std::io::Error> {
        let test_utility = crate::init::get_full_path(Utility::AptGet).await?;
        let test_update = crate::init::update(&test_utility).await?;
        assert!(test_update);
        let test_install = crate::init::install(&test_utility).await?;
        assert!(test_install);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn filesystem() -> Result<(), std::io::Error> {
        crate::init::filesystem().await?;
        let test_metadata_root = tokio::fs::metadata("/var/opt/attractor/").await?;
        assert!(test_metadata_root.is_dir());
        let test_metadata_target = tokio::fs::metadata("/var/opt/attractor/target").await?;
        assert!(test_metadata_target.is_dir());
        Ok(())
    }
}
