use crate::{Attractor, Utility};

use tokio::fs::create_dir_all;

async fn filesystem() -> Result<(), std::io::Error> {
    let dirs = vec![
        "/var/opt/attractor/",
        "/var/opt/attractor/minbase",
        "/var/opt/attractor/buildd",
        "/var/opt/attractor/fakechroot",
        "/var/opt/attractor/minbase/buster",
        "/var/opt/attractor/minbase/focal",
        "/var/opt/attractor/minbase/bionic",
        "/var/opt/attractor/minbase/xenial",
        "/var/opt/attractor/buildd/buster",
        "/var/opt/attractor/buildd/focal",
        "/var/opt/attractor/buildd/xenial",
        "/var/opt/attractor/fakechroot/buster",
        "/var/opt/attractor/fakechroot/focal",
        "/var/opt/attractor/fakechroot/bionic",
        "/var/opt/attractor/fakechroot/xenial",
        "/var/opt/attractor/target",
    ];

    for dir in dirs {
        create_dir_all(dir).await?;
    }

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
        let test_dirs = vec![
            "/var/opt/attractor/",
            "/var/opt/attractor/minbase",
            "/var/opt/attractor/buildd",
            "/var/opt/attractor/fakechroot",
            "/var/opt/attractor/minbase/buster",
            "/var/opt/attractor/minbase/focal",
            "/var/opt/attractor/minbase/bionic",
            "/var/opt/attractor/minbase/xenial",
            "/var/opt/attractor/buildd/buster",
            "/var/opt/attractor/buildd/focal",
            "/var/opt/attractor/buildd/xenial",
            "/var/opt/attractor/fakechroot/buster",
            "/var/opt/attractor/fakechroot/focal",
            "/var/opt/attractor/fakechroot/bionic",
            "/var/opt/attractor/fakechroot/xenial",
            "/var/opt/attractor/target",
        ];
        crate::init::filesystem().await?;
        for dir in test_dirs {
            let test_metadata_target = tokio::fs::metadata(dir).await?;
            assert!(test_metadata_target.is_dir());
        }
        Ok(())
    }
}
