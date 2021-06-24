use crate::{Attractor, Suite, Variant};

use tokio::process::Command;

async fn unpack_tarball(variant: Variant, code_name: Suite) -> Result<(), std::io::Error> {
    let unpack_tarball = variant.unpack_tarball().await;
    let code_name_kind = code_name.kind().await;
    let target = "/var/opt/attractor/target";
    let command = Command::new("/usr/sbin/debootstrap")
        .current_dir("/var/opt/attractor")
        .arg(unpack_tarball)
        .arg(code_name_kind)
        .arg(target)
        .status()
        .await?;
    Ok(())
}

impl Attractor {
    pub(crate) async fn run_install() -> Result<(), std::io::Error> {
        let variant = Variant::Minbase;
        let code_name = Suite::Buster;
        unpack_tarball(variant, code_name).await?;
        Ok(())
    }
}
