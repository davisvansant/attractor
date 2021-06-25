pub enum Variant {
    Minbase,
    Buildd,
    _Fakechroot,
}

impl Variant {
    pub async fn kind(&self) -> &str {
        match self {
            Variant::Minbase => "--variant=minbase",
            Variant::Buildd => "--variant=buildd",
            Variant::_Fakechroot => "--variant=fakechroot",
        }
    }

    pub async fn make_tarball(&self) -> &str {
        match self {
            Variant::Minbase => "--make-tarball=/var/opt/attractor/minbase.tar.gz",
            Variant::Buildd => "--make-tarball=/var/opt/attractor/buildd.tar.gz",
            Variant::_Fakechroot => "--make-tarball=/var/opt/attractor/fakechroot.tar.gz",
        }
    }

    pub async fn unpack_tarball(&self) -> &str {
        match self {
            Variant::Minbase => "--unpack-tarball=/var/opt/attractor/minbase.tar.gz",
            Variant::Buildd => "--unpack-tarball=/var/opt/attractor/buildd.tar.gz",
            Variant::_Fakechroot => "--unpack-tarball=/var/opt/attractor/fakechroot.tar.gz",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn kind_minbase() -> Result<(), std::io::Error> {
        let test_variant_minbase = Variant::Minbase;
        assert_eq!(test_variant_minbase.kind().await, "--variant=minbase");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn kind_buildd() -> Result<(), std::io::Error> {
        let test_variant_buildd = Variant::Buildd;
        assert_eq!(test_variant_buildd.kind().await, "--variant=buildd");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn kind_fakechroot() -> Result<(), std::io::Error> {
        let test_variant_fakechroot = Variant::_Fakechroot;
        assert_eq!(test_variant_fakechroot.kind().await, "--variant=fakechroot");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn make_tarball_minbase() -> Result<(), std::io::Error> {
        let test_variant_minbase = Variant::Minbase;
        assert_eq!(
            test_variant_minbase.make_tarball().await,
            "--make-tarball=/var/opt/attractor/minbase.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn make_tarball_buildd() -> Result<(), std::io::Error> {
        let test_variant_buildd = Variant::Buildd;
        assert_eq!(
            test_variant_buildd.make_tarball().await,
            "--make-tarball=/var/opt/attractor/buildd.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn make_tarball_fakechroot() -> Result<(), std::io::Error> {
        let test_variant_fakechroot = Variant::_Fakechroot;
        assert_eq!(
            test_variant_fakechroot.make_tarball().await,
            "--make-tarball=/var/opt/attractor/fakechroot.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn unpack_tarball_minbase() -> Result<(), std::io::Error> {
        let test_variant_minbase = Variant::Minbase;
        assert_eq!(
            test_variant_minbase.unpack_tarball().await,
            "--unpack-tarball=/var/opt/attractor/minbase.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn unpack_tarball_buildd() -> Result<(), std::io::Error> {
        let test_variant_buildd = Variant::Buildd;
        assert_eq!(
            test_variant_buildd.unpack_tarball().await,
            "--unpack-tarball=/var/opt/attractor/buildd.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn unpack_tarball_fakechroot() -> Result<(), std::io::Error> {
        let test_variant_fakechroot = Variant::_Fakechroot;
        assert_eq!(
            test_variant_fakechroot.unpack_tarball().await,
            "--unpack-tarball=/var/opt/attractor/fakechroot.tar.gz",
        );
        Ok(())
    }
}
