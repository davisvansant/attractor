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
}
