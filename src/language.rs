pub enum Language {
    Rust,
}

impl Language {
    pub async fn release(&self, version: &str) -> &str {
        match self {
            Language::Rust => match version {
                "1.53.0" => {
                    "https://static.rust-lang.org/dist/rust-1.53.0-x86_64-unknown-linux-gnu.tar.gz"
                }
                "1.52.1" => "1.52.1",
                "1.52.0" => "1.52.0",
                _ => panic!("Version unsupported!"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn language_rust_correct_version() -> Result<(), std::io::Error> {
        let test_language = Language::Rust;
        let test_language_release = test_language.release("1.53.0").await;
        assert_eq!(
            test_language_release,
            "https://static.rust-lang.org/dist/rust-1.53.0-x86_64-unknown-linux-gnu.tar.gz",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic(expected = "Version unsupported!")]
    async fn language_rust_incorrect_version() {
        let test_language = Language::Rust;
        test_language.release("1.49.0").await;
    }
}
