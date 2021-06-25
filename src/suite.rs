pub enum Suite {
    Buster,
    _Focal,
    _Bionic,
    _Xenial,
}

impl Suite {
    pub async fn kind(&self) -> &str {
        match self {
            Suite::Buster => "buster",
            Suite::_Focal => "focal",
            Suite::_Bionic => "bionic",
            Suite::_Xenial => "xenial",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn suite_kind_buster() -> Result<(), std::io::Error> {
        let test_suite_buster = Suite::Buster;
        assert_eq!(test_suite_buster.kind().await, "buster");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn suite_kind_focal() -> Result<(), std::io::Error> {
        let test_suite_buster = Suite::_Focal;
        assert_eq!(test_suite_buster.kind().await, "focal");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn suite_kind_bionic() -> Result<(), std::io::Error> {
        let test_suite_buster = Suite::_Bionic;
        assert_eq!(test_suite_buster.kind().await, "bionic");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn suite_kind_xenial() -> Result<(), std::io::Error> {
        let test_suite_buster = Suite::_Xenial;
        assert_eq!(test_suite_buster.kind().await, "xenial");
        Ok(())
    }
}
