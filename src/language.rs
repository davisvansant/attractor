use crate::{Suite, Utility};

use tokio::process::Command;

pub enum Language {
    Rust,
}

impl Language {
    pub async fn release(&self, version: &str) -> &str {
        match self {
            Language::Rust => match version {
                "1.53.0" => {
                    "https://static.rust-lang.org/dist/rust-1.53.0-x86_64-unknown-linux-gnu.tar.gz"
                }
                "1.52.1" => "1.52.1",
                "1.52.0" => "1.52.0",
                _ => panic!("Version unsupported!"),
            },
        }
    }

    pub async fn download(&self, release: &str) -> Result<bool, std::io::Error> {
        let tmp_path = "/var/opt/attractor/tmp";
        let wget = Utility::Wget.path().await?;

        let download = Command::new(&wget)
            .arg(release)
            .current_dir(tmp_path)
            .status()
            .await?;

        Ok(download.success())
    }

    pub async fn extract(&self, release: &str, suite: &Suite) -> Result<bool, std::io::Error> {
        let tmp_path = "/var/opt/attractor/tmp";
        let tar_path = Utility::Tar.path().await?;
        let release_path = release.rsplit_once('/').expect("failed to split!");
        let buildd = suite.buildd_path().await;

        let extract = Command::new(&tar_path)
            .arg("-xvf")
            .arg(release_path.1)
            .arg("-C")
            .arg(buildd)
            .current_dir(tmp_path)
            .status()
            .await?;

        Ok(extract.success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn language_rust_correct_version() -> Result<(), std::io::Error> {
        let test_language = Language::Rust;
        let test_language_release = test_language.release("1.53.0").await;
        assert_eq!(
            test_language_release,
            "https://static.rust-lang.org/dist/rust-1.53.0-x86_64-unknown-linux-gnu.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic(expected = "Version unsupported!")]
    async fn language_rust_incorrect_version() {
        let test_language = Language::Rust;
        test_language.release("1.49.0").await;
    }

    // #[tokio::test(flavor = "multi_thread")]
    // async fn language_rust_downloadand_extract() -> Result<(), std::io::Error> {
    //     let test_language = Language::Rust;
    //     let test_release = test_language.release("1.53.0").await;
    //     let test_language_download = test_language.download(test_release).await?;
    //     assert!(test_language_download);
    //     let test_download = "/var/opt/attractor/tmp/rust-1.53.0-x86_64-unknown-linux-gnu.tar.gz";
    //     let test_metadata_target = tokio::fs::metadata(test_download).await?;
    //     assert!(test_metadata_target.is_file());
    //     let test_suite = Suite::Buster;
    //     let test_language_extract = test_language.extract(test_release, &test_suite).await?;
    //     assert!(test_language_extract);
    //     let test_language_extract_path =
    //         "/var/opt/attractor/buildd/buster/rust-1.53.0-x86_64-unknown-linux-gnu";
    //     let test_metadata_target = tokio::fs::metadata(test_language_extract_path).await?;
    //     assert!(test_metadata_target.is_dir());
    //     Ok(())
    // }
}
