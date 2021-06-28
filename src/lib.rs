mod bootstrap;
mod init;
mod install;
mod language;
mod suite;
mod utility;
mod variant;

pub use language::Language;
pub use suite::Suite;
pub use utility::Utility;
pub use variant::Variant;

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
//
//     #[tokio::test(flavor = "multi_thread")]
//     async fn install() -> Result<(), std::io::Error> {
//         let test_attractor = Attractor::init().await?;
//         let test_attractor_install = test_attractor.install().await;
//         assert!(test_attractor_install.is_ok());
//         Ok(())
//     }
// }
