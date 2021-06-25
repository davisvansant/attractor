use tokio::process::Command;

pub enum Utility {
    AptGet,
    Debootstrap,
}

impl Utility {
    pub async fn name(&self) -> &str {
        match self {
            Utility::AptGet => "apt-get",
            Utility::Debootstrap => "debootstrap",
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_apt_get_name() -> Result<(), std::io::Error> {
        let test_utility_apt_get = Utility::AptGet;
        assert_eq!(test_utility_apt_get.name().await, "apt-get");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_debootstrap_name() -> Result<(), std::io::Error> {
        let test_utility_debootstrap = Utility::Debootstrap;
        assert_eq!(test_utility_debootstrap.name().await, "debootstrap");
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
    async fn utility_debootstrap_path() -> Result<(), std::io::Error> {
        let test_utility_debootstrap = Utility::Debootstrap;
        let test_utility_debootstrap_path = test_utility_debootstrap.path().await?;
        assert_eq!(
            test_utility_debootstrap_path.to_str().unwrap(),
            "/usr/sbin/debootstrap",
        );
        Ok(())
    }
}
