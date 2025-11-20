use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

use crate::command_runner::{CommandResult, CommandRunner};
use crate::rspec_runner::{RspecRunner};
use crate::file_path_parser::ParsedFilePath;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RspecServerArgs {
    #[schemars(
        description = "RSpec test file path (must end with '_spec.rb')",
        example = "spec/models/user_spec.rb"
    )]
    pub file: String,

    #[schemars(
        description = "Optional line numbers to target specific tests",
        example = "[37, 87]"
    )]
    pub line_numbers: Option<Vec<i32>>,
}

#[derive(Clone)]
pub struct RspecServer {
    tool_router: ToolRouter<RspecServer>,
    rspec_cmd: String,
}

#[tool_router]
impl RspecServer {
    pub fn new(rspec_cmd: String) -> Self {
        Self {
            tool_router: Self::tool_router(),
            rspec_cmd,
        }
    }

    #[tool(
        description = "Run RSpec tests for a specific file with optional line number targeting. Accepts file paths ending in '_spec.rb' with optional array of line numbers"
    )]
    async fn run_rspec(
        &self,
        Parameters(args): Parameters<RspecServerArgs>,
    ) -> Result<CallToolResult, McpError> {
        // Parse the file path and validate format
        let line_numbers = args.line_numbers.unwrap_or_default();
        let parsed_file = match ParsedFilePath::from_args(&args.file, line_numbers) {
            Ok(parsed) => parsed,
            Err(e) => {
                return Err(McpError::invalid_params(
                    format!("Invalid parameters: {}", e),
                    None,
                ));
            }
        };

        let runner = RspecRunner::new(self.rspec_cmd.clone());

        // Build the RSpec file argument from parsed components
        let rspec_arg = parsed_file.as_arg();

        match runner.run(&rspec_arg).await {
            Ok(CommandResult { exit_code, stdout, stderr }) => {
                let result_text = format!(
                    "Test Results for: {}\nExit Code: {}\n\nOutput:\n{}\n\nErrors:\n{}",
                    rspec_arg, exit_code, stdout, stderr
                );

                Ok(CallToolResult::success(vec![Content::text(result_text)]))
            }
            Err(e) => Err(McpError::internal_error(
                format!("Command failed: {}", e),
                None,
            )),
        }
    }
}

#[tool_handler]
impl ServerHandler for RspecServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Test runner server using configurable command. Tool: run_rspec (run tests for a file)."
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_rspec_tool() {
        let router = RspecServer::new("bundle exec rspec".to_string()).tool_router;

        let tools = router.list_all();
        assert_eq!(tools.len(), 1);

        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(tool_names.contains(&"run_rspec"));
    }

    #[test]
    fn test_test_runner_args_deserialization() {
        let json = r#"
        {
            "file": "spec/models/user_spec.rb"
        }
        "#;

        let args: RspecServerArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.file, "spec/models/user_spec.rb");
        assert_eq!(args.line_numbers, None);
    }

    #[test]
    fn test_test_runner_args_with_line_numbers() {
        let json = r#"
        {
            "file": "spec/models/user_spec.rb",
            "line_numbers": [37, 87]
        }
        "#;

        let args: RspecServerArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.file, "spec/models/user_spec.rb");
        assert_eq!(args.line_numbers, Some(vec![37, 87]));
    }
}
