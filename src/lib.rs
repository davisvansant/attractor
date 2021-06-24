mod bootstrap;
mod init;

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
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test(flavor = "multi_thread")]
//     async fn init() -> Result<(), std::io::Error> {
//         let test_attractor = Attractor::init().await;
//         assert!(test_attractor.is_ok());
//         Ok(())
//     }
//
//     #[tokio::test(flavor = "multi_thread")]
//     async fn bootstrap() -> Result<(), std::io::Error> {
//         let test_attractor = Attractor::init().await?;
//         let test_attractor_bootstrap = test_attractor.bootstrap().await;
//         assert!(test_attractor_bootstrap.is_ok());
//         Ok(())
//     }
// }
