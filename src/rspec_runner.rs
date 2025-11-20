use async_trait::async_trait;
use tokio::process::Command;

use crate::command_runner::{CommandResult, CommandRunner};

#[derive(Clone)]
pub struct RspecRunner {
    cmd: String,
    args: Vec<String>,
}

impl RspecRunner {
    pub fn new(command: String) -> Self {
        let parts: Vec<String> = command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let cmd = parts.first()
            .expect("Command cannot be empty")
            .to_string();

        let args = parts.get(1..).unwrap_or(&[]).to_vec();

        Self { cmd, args }
    }
}

#[async_trait]
impl CommandRunner for RspecRunner {
    async fn run(&self, file_path: &str) -> Result<CommandResult, String> {

        let mut cmd = Command::new(&self.cmd);

        for arg in &self.args {
            cmd.arg(arg);
        }

        cmd.arg(file_path);

        match cmd.output().await {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);

                Ok(CommandResult {
                    exit_code,
                    stdout,
                    stderr,
                })
            }
            Err(e) => Err(format!("Command execution failed: {}", e)),
        }
    }
}
