use crate::Suite;

use std::path::PathBuf;
use std::str::FromStr;

use tokio::process::Command;

pub struct Chroot {
    pub directory: PathBuf,
}

impl Chroot {
    pub async fn build(kind: Suite) -> Result<Chroot, Box<dyn std::error::Error>> {
        let path = kind.buildd_path().await;
        let directory = PathBuf::from_str(path)?;

        Ok(Chroot { directory })
    }

    pub async fn run(&self, _command: &str) -> Result<bool, std::io::Error> {
        let run = Command::new("chroot")
            .arg("/bin/echo")
            .arg("hello")
            .current_dir("/var/opt/attractor/buildd")
            .status()
            .await?;

        Ok(run.success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn build_buster() -> Result<(), std::io::Error> {
        let test_suite = Suite::Buster;
        let test_chroot = Chroot::build(test_suite).await;
        assert!(test_chroot.is_ok());
        assert_eq!(
            test_chroot.unwrap().directory.to_str().unwrap(),
            "/var/opt/attractor/buildd/buster",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_focal() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Focal;
        let test_chroot = Chroot::build(test_suite).await;
        assert!(test_chroot.is_ok());
        assert_eq!(
            test_chroot.unwrap().directory.to_str().unwrap(),
            "/var/opt/attractor/buildd/focal",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_bionic() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Bionic;
        let test_chroot = Chroot::build(test_suite).await;
        assert!(test_chroot.is_ok());
        assert_eq!(
            test_chroot.unwrap().directory.to_str().unwrap(),
            "/var/opt/attractor/buildd/bionic",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_xenial() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Xenial;
        let test_chroot = Chroot::build(test_suite).await;
        assert!(test_chroot.is_ok());
        assert_eq!(
            test_chroot.unwrap().directory.to_str().unwrap(),
            "/var/opt/attractor/buildd/xenial",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn run() -> Result<(), Box<dyn std::error::Error>> {
        tokio::fs::create_dir_all("/var/opt/attractor/buildd").await?;
        let test_suite = Suite::Buster;
        let test_chroot = Chroot::build(test_suite).await?;
        let test_run = test_chroot.run("todo").await?;
        assert!(!test_run);
        Ok(())
    }
}
