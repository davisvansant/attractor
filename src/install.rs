use crate::{Attractor, Suite, Utility, Variant};

use tokio::process::Command;

async fn unpack_tarball(variant: Variant, suite: Suite) -> Result<bool, std::io::Error> {
    let unpack_tarball = variant.unpack_tarball().await;
    let name = variant.name().await;
    let code_name = suite.code_name().await;
    let target = "/var/opt/attractor/target".to_owned() + name + code_name;
    let command_path = Utility::Debootstrap.path().await?;
    let command = Command::new(&command_path)
        .current_dir("/var/opt/attractor")
        .arg(unpack_tarball)
        .arg(code_name)
        .arg(target)
        .status()
        .await?;
    Ok(command.success())
}

impl Attractor {
    pub(crate) async fn run_install() -> Result<(), std::io::Error> {
        let variant = Variant::Minbase;
        let code_name = Suite::Buster;
        unpack_tarball(variant, code_name).await?;
        Ok(())
    }
}
