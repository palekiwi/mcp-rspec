use clap::Parser;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use std::net::SocketAddr;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

mod rspec_runner;
mod command_runner;
mod file_path_parser;
mod rspec_server;

#[cfg(test)]
mod mock_runner;

use crate::rspec_runner::RspecRunner;
use crate::rspec_server::RspecServer;

#[derive(Parser, Debug)]
#[command(name = "mcp-rspec")]
#[command(about = "Configurable RSpec runner MCP server over HTTP with SSE")]
#[command(version)]
struct Cli {
    #[arg(short = 'H', long, env = "MCP_RSPEC_HOSTNAME", default_value = "127.0.0.1")]
    hostname: String,

    #[arg(short, long, env = "MCP_RSPEC_PORT", default_value = "30301")]
    port: u16,

    #[arg(short = 'c', long, env = "RSPEC_RUNNER_CMD", default_value = "bundle exec rspec")]
    rspec_cmd: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.rspec_cmd.trim().is_empty() {
        anyhow::bail!("rspec_cmd cannot be empty")
    }

    let bind_address: SocketAddr = format!("{}:{}", cli.hostname, cli.port).parse()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting MCP RSpec server on {}", bind_address);

    let config = SseServerConfig {
        bind: bind_address,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    let (sse_server, router) = SseServer::new(config);

    let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;

    let ct = sse_server.config.ct.child_token();

    let server = axum::serve(listener, router).with_graceful_shutdown(async move {
        ct.cancelled().await;
        tracing::info!("sse server cancelled");
    });

    tokio::spawn(async move {
        if let Err(e) = server.await {
            tracing::error!(error = %e, "sse server shutdown with error");
        }
    });

    let runner = RspecRunner::new(cli.rspec_cmd);
    let ct = sse_server.with_service(move || RspecServer::new(runner.clone()));

    tracing::info!("MCP RSpec server is running!");
    tracing::info!("SSE endpoint: http://{}/sse", bind_address);
    tracing::info!("Message endpoint: http://{}/message", bind_address);
    tracing::info!("Press Ctrl+C to stop");

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
