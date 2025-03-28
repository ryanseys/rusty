# Rusty

A benchmarking project comparing Rust and Ruby implementations of Fibonacci number calculation, with detailed performance metrics and AI-powered analysis.

## Features

- Calculate Fibonacci numbers using both Rust and Ruby implementations
- Comprehensive performance benchmarking with 100 iterations per implementation
- Statistical analysis including mean, median, P95, min, and max timings
- AI-powered performance comparison using local Ollama instance
- Beautiful terminal output with tables and colored text
- MCP server mode for metrics collection and reporting

## Prerequisites

1. Rust and Cargo
2. Ruby
3. Ollama running locally with the `mistral` model (or modify the code to use a different model)

## Quick Start

1. Clone this repository
2. Build and run:

```bash
cargo run -- -N 40  # Calculate 40th Fibonacci number
```

## Usage

```bash
cargo run -- [OPTIONS]

Options:
  -N, --number <N>    Fibonacci number to calculate (default: 10)
      --rust         Use only Rust implementation
      --ruby         Use only Ruby implementation
      --metrics     Enable MCP server mode
  -h, --help         Print help
  -V, --version      Print version
```

### Example Commands

1. Run both implementations (default):

```bash
cargo run -- -N 40
```

2. Run just the Rust implementation:

```bash
cargo run -- --rust -N 40
```

3. Run just the Ruby implementation:

```bash
cargo run -- --ruby -N 40
```

4. Run with MCP server enabled:

```bash
cargo run -- -N 40 --metrics
```

### Output Example

The program provides:

- Detailed iteration-by-iteration timing for both implementations
- Statistical analysis in table format
- Performance comparison with exact speed ratios
- AI-generated playful comparison of the results
- JSON metrics output when running in MCP server mode

## MCP Server Mode

The project includes a Model Context Protocol (MCP) server that can be used to collect and report metrics about the Fibonacci calculations. This is particularly useful for integrating with other tools or collecting performance data over time.

### Running in MCP Server Mode

To start the MCP server, use the `--metrics` flag:

```bash
cargo run -- --metrics
```

The server will start and wait for input on stdin. You can then send requests in JSON format:

```bash
# Example: Calculate request
echo '{"type":"Calculate","data":{"number":40,"implementation":"rust"}}' | cargo run -- --metrics

# Example: Benchmark request
echo '{"type":"Benchmark","data":{"number":40,"implementation":"rust"}}' | cargo run -- --metrics
```

### Server Protocol

The server accepts two types of requests:

1. Calculate Request (single calculation):

```json
{
  "type": "Calculate",
  "data": {
    "number": 40,
    "implementation": "rust"  // Can be: "rust", "ruby", "rust_ruby_ffi", or "ruby_rust_ffi"
  }
}
```

2. Benchmark Request (runs 100 iterations):

```json
{
  "type": "Benchmark",
  "data": {
    "number": 40,
    "implementation": "rust"
  }
}
```

### Server Responses

The server responds with JSON in one of two formats:

1. Successful Result:

```json
{
  "type": "Result",
  "data": {
    "result": 102334155,
    "execution_time_ms": 0.123,
    "implementation": "rust"
  }
}
```

2. Error:

```json
{
  "type": "Error",
  "data": "Invalid implementation specified"
}
```

### Comparison Metrics

When running benchmarks, the server also outputs comprehensive comparison metrics:

```json
{
  "rust_metrics": {
    "number": 40,
    "result": 102334155,
    "execution_time_ms": 0.123,
    "implementation": "Pure Rust"
  },
  "ruby_metrics": {
    "number": 40,
    "result": 102334155,
    "execution_time_ms": 1.234,
    "implementation": "Pure Ruby"
  },
  "rust_ruby_ffi_metrics": {
    "number": 40,
    "result": 102334155,
    "execution_time_ms": 0.456,
    "implementation": "Ruby->Rust FFI"
  },
  "ruby_rust_ffi_metrics": {
    "number": 40,
    "result": 102334155,
    "execution_time_ms": 0.789,
    "implementation": "Rust->Ruby FFI"
  },
  "speedup_vs_rust": 10.032
}
```

## Implementation Details

### Rust Implementation

- Direct implementation in Rust
- Uses u64 for Fibonacci numbers
- Includes black_box to prevent compiler optimizations
- 100 iterations for accurate timing statistics

### Ruby Implementation

- Pure Ruby implementation
- Runs via subprocess for clean separation
- 100 iterations matching Rust implementation
- Separate process ensures fair comparison

### Performance Analysis

- Comprehensive statistics (mean, median, P95, min, max)
- Pretty-printed tables for easy reading
- AI-powered analysis for human-friendly comparison
- Accurate timing down to microseconds

## Project Structure

```
.
├── src/
│   ├── main.rs         # Main Rust implementation and benchmarking
│   └── mcp_server.rs   # MCP server implementation
├── fibonacci.rb        # Ruby implementation
├── fibonacci_ffi.rb    # Ruby FFI implementation
├── Cargo.toml         # Rust dependencies
└── README.md         # This file
```

## Dependencies

### Rust

- `clap` - Command-line argument parsing
- `colored` - Terminal output formatting
- `criterion` - Performance benchmarking
- `prettytable-rs` - Table formatting
- `tokio` - Async runtime for Ollama
- `ollama-rs` - Ollama AI integration
- `serde` - JSON serialization/deserialization
- `serde_json` - JSON support

### External

- Ruby (any recent version)
- Ollama running locally with mistral model

## Notes

- The program runs each implementation 100 times for statistical significance
- Timing includes process startup time for Ruby
- AI comparison requires a running Ollama instance
- Falls back to simple comparison if Ollama is unavailable
- MCP server mode uses JSON for all communication
- Server responses are line-delimited JSON objects
