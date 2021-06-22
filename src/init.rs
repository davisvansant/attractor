use crate::Attractor;

use std::path::{Path, PathBuf};

use tokio::fs::create_dir_all;
use tokio::process::Command;

async fn get_full_path() -> Result<PathBuf, std::io::Error> {
    let mut path_buf = PathBuf::with_capacity(15);
    let command = Command::new("/usr/bin/which").arg("dpkg").output().await?;
    let string = String::from_utf8(command.stdout).expect("Failed to convert to String!");

    path_buf.push(string.trim());

    Ok(path_buf)
}

async fn check(path: &Path) -> Result<bool, std::io::Error> {
    let command = Command::new(path)
        .arg("-s")
        .arg("debootstrap")
        .status()
        .await?;

    Ok(command.success())
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
    let dir = "/var/opt/attractor/";

    create_dir_all(dir).await?;

    Ok(())
}

impl Attractor {
    pub async fn prep() -> Result<(), std::io::Error> {
        let full_path = crate::init::get_full_path().await?;
        let check = crate::init::check(&full_path).await?;

        if !check {
            crate::init::update(&full_path).await?;
            crate::init::install(&full_path).await?;
        }

        filesystem().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn get_full_path() -> Result<(), std::io::Error> {
        let test_path_buf = crate::init::get_full_path().await?;
        assert_eq!(test_path_buf.to_str().unwrap(), "/usr/bin/dpkg");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn check() -> Result<(), std::io::Error> {
        let test_path = Path::new("/usr/bin/dpkg");
        let test_check = crate::init::check(test_path).await?;
        assert!(!test_check);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn update() -> Result<(), std::io::Error> {
        let test_path = Path::new("/usr/bin/apt-get");
        let test_check = crate::init::update(test_path).await?;
        assert!(test_check);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn install() -> Result<(), std::io::Error> {
        let test_path = Path::new("/usr/bin/apt-get");
        let test_install = crate::init::install(test_path).await?;
        if !test_install {
            crate::init::update(test_path).await?;
            assert!(crate::init::install(test_path).await?);
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn filesystem() -> Result<(), std::io::Error> {
        crate::init::filesystem().await?;
        let test_metadata = tokio::fs::metadata("/var/opt/attractor/").await?;
        assert!(test_metadata.is_dir());
        Ok(())
    }
}
