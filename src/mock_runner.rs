use async_trait::async_trait;
use crate::command_runner::{CommandResult, CommandRunner};

#[derive(Clone)]
pub struct MockRunner {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

impl MockRunner {
    pub fn new() -> Self {
        Self {
            exit_code: 0,
            stdout: "mock test output".to_string(),
            stderr: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_result(exit_code: i32, stdout: String, stderr: String) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
        }
    }
}

#[async_trait]
impl CommandRunner for MockRunner {
    async fn run(&self, _path: &str) -> Result<CommandResult, String> {
        Ok(CommandResult {
            exit_code: self.exit_code,
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
        })
    }
}
