pub enum Suite {
    Buster,
    _Focal,
    _Bionic,
    _Xenial,
}

impl Suite {
    pub async fn code_name(&self) -> &str {
        match self {
            Suite::Buster => "buster",
            Suite::_Focal => "focal",
            Suite::_Bionic => "bionic",
            Suite::_Xenial => "xenial",
        }
    }

    pub async fn buildd_path(&self) -> &str {
        match self {
            Suite::Buster => "/var/opt/attractor/buildd/buster",
            Suite::_Focal => "/var/opt/attractor/buildd/focal",
            Suite::_Bionic => "/var/opt/attractor/buildd/bionic",
            Suite::_Xenial => "/var/opt/attractor/buildd/xenial",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn code_name_buster() -> Result<(), std::io::Error> {
        let test_suite = Suite::Buster;
        assert_eq!(test_suite.code_name().await, "buster");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn code_name_focal() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Focal;
        assert_eq!(test_suite.code_name().await, "focal");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn code_name_bionic() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Bionic;
        assert_eq!(test_suite.code_name().await, "bionic");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn code_name_xenial() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Xenial;
        assert_eq!(test_suite.code_name().await, "xenial");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn buildd_path_buster() -> Result<(), std::io::Error> {
        let test_suite = Suite::Buster;
        assert_eq!(
            test_suite.buildd_path().await,
            "/var/opt/attractor/buildd/buster",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn buildd_path_focal() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Focal;
        assert_eq!(
            test_suite.buildd_path().await,
            "/var/opt/attractor/buildd/focal",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn buildd_path_bionic() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Bionic;
        assert_eq!(
            test_suite.buildd_path().await,
            "/var/opt/attractor/buildd/bionic",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn buildd_path_xenial() -> Result<(), std::io::Error> {
        let test_suite = Suite::_Xenial;
        assert_eq!(
            test_suite.buildd_path().await,
            "/var/opt/attractor/buildd/xenial",
        );
        Ok(())
    }
}
