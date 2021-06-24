mod bootstrap;
mod init;

enum Suite {
    Buster,
    _Focal,
    _Bionic,
    _Xenial,
}

impl Suite {
    pub(crate) async fn kind(&self) -> &str {
        match self {
            Suite::Buster => "buster",
            Suite::_Focal => "focal",
            Suite::_Bionic => "bionic",
            Suite::_Xenial => "xenial",
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
