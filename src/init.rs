use crate::{Attractor, Utility};

use tokio::fs::create_dir_all;

async fn filesystem() -> Result<(), std::io::Error> {
    let dir = "/var/opt/attractor/target";

    create_dir_all(dir).await?;

    Ok(())
}

impl Attractor {
    pub(crate) async fn run_init() -> Result<(), std::io::Error> {
        let apt_get = Utility::AptGet;

        apt_get.update().await?;
        apt_get.install().await?;

        filesystem().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn filesystem() -> Result<(), std::io::Error> {
        crate::init::filesystem().await?;
        let test_metadata_root = tokio::fs::metadata("/var/opt/attractor/").await?;
        assert!(test_metadata_root.is_dir());
        let test_metadata_target = tokio::fs::metadata("/var/opt/attractor/target").await?;
        assert!(test_metadata_target.is_dir());
        Ok(())
    }
}
