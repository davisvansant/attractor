use crate::Suite;

use std::path::PathBuf;
use std::str::FromStr;

pub struct Chroot {
    pub directory: PathBuf,
}

impl Chroot {
    pub async fn build(kind: Suite) -> Result<Chroot, Box<dyn std::error::Error>> {
        let path = kind.buildd_path().await;
        let directory = PathBuf::from_str(path)?;

        Ok(Chroot { directory })
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
}
