mod init;

pub struct Attractor {}

impl Attractor {
    pub async fn init() -> Result<Attractor, std::io::Error> {
        Self::prep().await?;
        Ok(Attractor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), std::io::Error> {
        Attractor::init().await?;
        Ok(())
    }
}
