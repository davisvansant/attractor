use tokio::process::Command;

pub enum Utility {
    AptGet,
    Chroot,
    Debootstrap,
    Tar,
    Wget,
}

impl Utility {
    pub async fn name(&self) -> &str {
        match self {
            Utility::AptGet => "apt-get",
            Utility::Chroot => "chroot",
            Utility::Debootstrap => "debootstrap",
            Utility::Tar => "tar",
            Utility::Wget => "wget",
        }
    }

    pub async fn path(&self) -> Result<std::ffi::OsString, std::io::Error> {
        let utility_name = self.name().await;
        let command = Command::new("/usr/bin/which")
            .arg(utility_name)
            .output()
            .await?;
        let string = String::from_utf8(command.stdout).expect("Failed to convert to String!");

        Ok(std::ffi::OsString::from(string.trim()))
    }

    pub async fn update(&self) -> Result<bool, std::io::Error> {
        match self {
            Utility::AptGet => {
                let command_path = self.path().await?;
                let command = Command::new(command_path).arg("update").status().await?;
                Ok(command.success())
            }
            // Utility::Debootstrap => {
            //     println!("Utility - debootstrap cannot run update!");
            //     Ok(false)
            // }
            _ => {
                println!("{} cannot run apt-get update!", self.name().await);
                Ok(false)
            }
        }
    }

    pub async fn install(&self) -> Result<bool, std::io::Error> {
        match self {
            // Utility::AptGet => {
            //     println!("Utility - apt-get is already installed!");
            //     Ok(false)
            // }
            Utility::Debootstrap => {
                let command_path = Utility::AptGet.path().await?;
                let command = Command::new(command_path)
                    .arg("install")
                    .arg(self.name().await)
                    .arg("-y")
                    .status()
                    .await?;

                Ok(command.success())
            }
            _ => {
                println!("{} is already installed!", self.name().await);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn apt_get_update_and_install() -> Result<(), std::io::Error> {
        let test_utility = Utility::AptGet;
        let test_apt_get_update = test_utility.update().await?;
        assert!(test_apt_get_update);
        let test_apt_get_install = test_utility.install().await?;
        assert!(!test_apt_get_install);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn chroot_update_and_install() -> Result<(), std::io::Error> {
        let test_utility = Utility::Chroot;
        let test_chroot_update = test_utility.update().await?;
        assert!(!test_chroot_update);
        let test_chroot_install = test_utility.install().await?;
        assert!(!test_chroot_install);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn debootstrap_update_and_install() -> Result<(), std::io::Error> {
        let test_utility = Utility::Debootstrap;
        let test_debootstrap_update = test_utility.update().await?;
        assert!(!test_debootstrap_update);
        let test_debootstrap_install = test_utility.install().await?;
        assert!(test_debootstrap_install);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn wget_update_and_install() -> Result<(), std::io::Error> {
        let test_utility = Utility::Wget;
        let test_update = test_utility.update().await?;
        assert!(!test_update);
        let test_install = test_utility.install().await?;
        assert!(!test_install);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_apt_get_name() -> Result<(), std::io::Error> {
        let test_utility_apt_get = Utility::AptGet;
        assert_eq!(test_utility_apt_get.name().await, "apt-get");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_chroot_name() -> Result<(), std::io::Error> {
        let test_utility_chroot = Utility::Chroot;
        assert_eq!(test_utility_chroot.name().await, "chroot");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_debootstrap_name() -> Result<(), std::io::Error> {
        let test_utility_debootstrap = Utility::Debootstrap;
        assert_eq!(test_utility_debootstrap.name().await, "debootstrap");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_tar_name() -> Result<(), std::io::Error> {
        let test_utility = Utility::Tar;
        assert_eq!(test_utility.name().await, "tar");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_wget_name() -> Result<(), std::io::Error> {
        let test_utility = Utility::Wget;
        assert_eq!(test_utility.name().await, "wget");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_apt_get_path() -> Result<(), std::io::Error> {
        let test_utility_apt_get = Utility::AptGet;
        let test_utility_apt_get_path = test_utility_apt_get.path().await?;
        assert_eq!(
            test_utility_apt_get_path.to_str().unwrap(),
            "/usr/bin/apt-get",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_chroot_path() -> Result<(), std::io::Error> {
        let test_utility_chroot = Utility::Chroot;
        let test_utility_chroot_path = test_utility_chroot.path().await?;
        assert_eq!(
            test_utility_chroot_path.to_str().unwrap(),
            "/usr/sbin/chroot",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_debootstrap_path() -> Result<(), std::io::Error> {
        let test_utility_debootstrap = Utility::Debootstrap;
        let test_utility_debootstrap_path = test_utility_debootstrap.path().await?;
        assert_eq!(
            test_utility_debootstrap_path.to_str().unwrap(),
            "/usr/sbin/debootstrap",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_tar_path() -> Result<(), std::io::Error> {
        let test_utility = Utility::Tar;
        let test_utility_path = test_utility.path().await?;
        assert_eq!(test_utility_path.to_str().unwrap(), "/bin/tar");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_wget_path() -> Result<(), std::io::Error> {
        let test_utility = Utility::Wget;
        let test_utility_path = test_utility.path().await?;
        assert_eq!(test_utility_path.to_str().unwrap(), "/usr/bin/wget");
        Ok(())
    }
}
