pub enum ChrootCommand {
    Echo,
}

impl ChrootCommand {
    pub async fn name(&self) -> &str {
        match self {
            ChrootCommand::Echo => "echo",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn command_name_echo() -> Result<(), std::io::Error> {
        let test_command = ChrootCommand::Echo;
        let test_command_name = test_command.name().await;
        assert_eq!(test_command_name, "echo");
        Ok(())
    }
}
