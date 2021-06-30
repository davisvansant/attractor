pub struct Chroot {}

impl Chroot {
    pub async fn build() -> Result<Chroot, std::io::Error> {
        Ok(Chroot {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn build() -> Result<(), std::io::Error> {
        let test_chroot = Chroot::build().await;
        assert!(test_chroot.is_ok());
        Ok(())
    }
}
