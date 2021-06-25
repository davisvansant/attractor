mod bootstrap;
mod init;
mod install;

use tokio::process::Command;

enum Suite {
    Buster,
    _Focal,
    _Bionic,
    _Xenial,
}

impl Suite {
    async fn kind(&self) -> &str {
        match self {
            Suite::Buster => "buster",
            Suite::_Focal => "focal",
            Suite::_Bionic => "bionic",
            Suite::_Xenial => "xenial",
        }
    }
}

enum Utility {
    AptGet,
    Debootstrap,
}

impl Utility {
    async fn name(&self) -> &str {
        match self {
            Utility::AptGet => "apt-get",
            Utility::Debootstrap => "debootstrap",
        }
    }

    async fn path(&self) -> Result<std::ffi::OsString, std::io::Error> {
        let utility_name = self.name().await;
        let command = Command::new("/usr/bin/which")
            .arg(utility_name)
            .output()
            .await?;
        let string = String::from_utf8(command.stdout).expect("Failed to convert to String!");

        Ok(std::ffi::OsString::from(string.trim()))
    }
}

enum Variant {
    Minbase,
    Buildd,
    _Fakechroot,
}

impl Variant {
    async fn kind(&self) -> &str {
        match self {
            Variant::Minbase => "--variant=minbase",
            Variant::Buildd => "--variant=buildd",
            Variant::_Fakechroot => "--variant=fakechroot",
        }
    }

    async fn make_tarball(&self) -> &str {
        match self {
            Variant::Minbase => "--make-tarball=/var/opt/attractor/minbase.tar.gz",
            Variant::Buildd => "--make-tarball=/var/opt/attractor/buildd.tar.gz",
            Variant::_Fakechroot => "--make-tarball=/var/opt/attractor/fakechroot.tar.gz",
        }
    }

    async fn unpack_tarball(&self) -> &str {
        match self {
            Variant::Minbase => "--unpack-tarball=/var/opt/attractor/minbase.tar.gz",
            Variant::Buildd => "--unpack-tarball=/var/opt/attractor/buildd.tar.gz",
            Variant::_Fakechroot => "--unpack-tarball=/var/opt/attractor/fakechroot.tar.gz",
        }
    }
}

pub struct Attractor {}

impl Attractor {
    pub async fn init() -> Result<Attractor, std::io::Error> {
        Self::run_init().await?;
        Ok(Attractor {})
    }

    pub async fn bootstrap(&self) -> Result<(), std::io::Error> {
        Self::run_bootrap().await?;
        Ok(())
    }

    pub async fn install(&self) -> Result<(), std::io::Error> {
        Self::run_install().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn utility_path() -> Result<(), std::io::Error> {
        let test_utility = Utility::AptGet;
        let test_utility_path = test_utility.path().await?;
        assert_eq!(test_utility_path, "/usr/bin/apt-get");
        Ok(())
    }

    // #[tokio::test(flavor = "multi_thread")]
    // async fn init() -> Result<(), std::io::Error> {
    //     let test_attractor = Attractor::init().await;
    //     assert!(test_attractor.is_ok());
    //     Ok(())
    // }
    //
    // #[tokio::test(flavor = "multi_thread")]
    // async fn bootstrap() -> Result<(), std::io::Error> {
    //     let test_attractor = Attractor::init().await?;
    //     let test_attractor_bootstrap = test_attractor.bootstrap().await;
    //     assert!(test_attractor_bootstrap.is_ok());
    //     Ok(())
    // }
}
