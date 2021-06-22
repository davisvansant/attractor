pub struct Attractor {}

impl Attractor {
    pub async fn init() -> Attractor {
        Attractor {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() {
        Attractor::init().await;
    }
}
