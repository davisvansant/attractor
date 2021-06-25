use crate::{Attractor, Suite, Utility, Variant};

use tokio::process::Command;

async fn make_tarball(variant: Variant, code_name: Suite) -> Result<bool, std::io::Error> {
    let variant_kind = variant.kind().await;
    let make_tarball = variant.make_tarball().await;
    let code_name_kind = code_name.kind().await;
    let target = "/var/opt/attractor/target";
    let command_path = Utility::Debootstrap.path().await?;
    let command = Command::new(&command_path)
        .current_dir("/var/opt/attractor")
        .arg(variant_kind)
        .arg(make_tarball)
        .arg(code_name_kind)
        .arg(target)
        .status()
        .await?;

    Ok(command.success())
}

impl Attractor {
    pub(crate) async fn run_bootrap() -> Result<(), std::io::Error> {
        make_tarball(Variant::Minbase, Suite::Buster).await?;
        make_tarball(Variant::Buildd, Suite::Buster).await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test(flavor = "multi_thread")]
//     async fn make_tarball_minbase() -> Result<(), std::io::Error> {
//         Command::new("apt-get").arg("update").status().await?;
//         Command::new("apt-get")
//             .arg("install")
//             .arg("debootstrap")
//             .arg("-y")
//             .status()
//             .await?;
//         tokio::fs::create_dir_all("/var/opt/attractor/target").await?;
//         let test_variant_minbase =
//             crate::bootstrap::make_tarball(Variant::Minbase, Suite::Buster).await?;
//         assert!(test_variant_minbase);
//         Ok(())
//     }
//
//     #[tokio::test(flavor = "multi_thread")]
//     async fn make_tarball_buildd() -> Result<(), std::io::Error> {
//         Command::new("apt-get").arg("update").status().await?;
//         Command::new("apt-get")
//             .arg("install")
//             .arg("debootstrap")
//             .arg("-y")
//             .status()
//             .await?;
//         tokio::fs::create_dir_all("/var/opt/attractor/target").await?;
//         let test_variant_minbase =
//             crate::bootstrap::make_tarball(Variant::Buildd, Suite::Buster).await?;
//         assert!(test_variant_minbase);
//         Ok(())
//     }
// }
