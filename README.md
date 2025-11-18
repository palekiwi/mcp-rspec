# MCP RSpec

A Model Context Protocol (MCP) server that provides configurable RSpec test execution over HTTP with Server-Sent Events (SSE).

## Features

- **Configurable RSpec Command**: Customize the RSpec command (default: `bundle exec rspec`)
- **File-Specific Testing**: Run tests for specific RSpec files (`*_spec.rb`)
- **Line Number Targeting**: Execute tests at specific line numbers for precise test targeting
- **Security Validation**: Built-in path traversal prevention and file format validation
- **HTTP/SSE Transport**: Uses Server-Sent Events for real-time communication
- **Graceful Shutdown**: Clean shutdown with signal handling

## Installation

### From Source

```bash
git clone <repository-url>
cd mcp-rspec
cargo build --release
```

### Using Nix (Flake)

```bash
nix build
```

## Usage

### Starting the Server

```bash
# Using defaults (127.0.0.1:30301, bundle exec rspec)
mcp-rspec

# Custom host and port
mcp-rspec -H 0.0.0.0 -p 8080

# Custom RSpec command
mcp-rspec -c "rspec"

# Using environment variables
MCP_RSPEC_HOSTNAME=0.0.0.0 MCP_RSPEC_PORT=8080 RSPEC_RUNNER_CMD="bundle exec rspec" mcp-rspec
```

### Available Tools

#### `run_rspec`

Run RSpec tests for a specific file with optional line number targeting.

**Parameters:**
- `file` (string, required): RSpec test file path (must end with `_spec.rb`)
- `line_numbers` (array, optional): Line numbers to target specific tests

**Examples:**

```json
// Run all tests in a file
{
  "file": "spec/models/user_spec.rb"
}

// Run tests at specific lines
{
  "file": "spec/models/user_spec.rb",
  "line_numbers": [37, 87]
}
```

### Server Endpoints

Once started, the server provides:
- **SSE Endpoint**: `http://host:port/sse` - For real-time event streaming
- **Message Endpoint**: `http://host:port/message` - For sending MCP messages

## Configuration

### Command Line Options

| Option | Short | Environment Variable | Default | Description |
|--------|-------|----------------------|---------|-------------|
| `--hostname` | `-H` | `MCP_RSPEC_HOSTNAME` | `127.0.0.1` | Server bind address |
| `--port` | `-p` | `MCP_RSPEC_PORT` | `30301` | Server port |
| `--rspec-cmd` | `-c` | `RSPEC_RUNNER_CMD` | `bundle exec rspec` | RSpec command to execute |

### Security Features

- **Path Traversal Prevention**: Blocks `../` sequences in file paths
- **File Format Validation**: Only allows files ending with `_spec.rb`
- **Input Sanitization**: Validates against null bytes and dangerous characters
- **Line Number Validation**: Ensures line numbers are positive integers

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build
```

### Running in Development Mode

```bash
cargo run
```

## Dependencies

- `rmcp` - MCP server implementation with SSE transport
- `tokio` - Async runtime
- `axum` - HTTP web framework
- `serde` - Serialization/deserialization
- `clap` - Command line argument parsing
- `tracing` - Structured logging

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
