use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[async_trait]
pub trait CommandRunner: Send + Sync {
    async fn run(&self, path: &str) -> Result<CommandResult, String>;
}
