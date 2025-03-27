# Rusty

A benchmarking project comparing Rust and Ruby implementations of Fibonacci number calculation, with detailed performance metrics and AI-powered analysis.

## Features

- Calculate Fibonacci numbers using both Rust and Ruby implementations
- Comprehensive performance benchmarking with 100 iterations per implementation
- Statistical analysis including mean, median, P95, min, and max timings
- AI-powered performance comparison using local Ollama instance
- Beautiful terminal output with tables and colored text

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

### Output Example

The program provides:

- Detailed iteration-by-iteration timing for both implementations
- Statistical analysis in table format
- Performance comparison with exact speed ratios
- AI-generated playful comparison of the results

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
│   └── main.rs      # Main Rust implementation and benchmarking
├── fibonacci.rb     # Ruby implementation
├── Cargo.toml       # Rust dependencies
└── README.md       # This file
```

## Dependencies

### Rust

- `clap` - Command-line argument parsing
- `colored` - Terminal output formatting
- `criterion` - Performance benchmarking
- `prettytable-rs` - Table formatting
- `tokio` - Async runtime for Ollama
- `ollama-rs` - Ollama AI integration

### External

- Ruby (any recent version)
- Ollama running locally with mistral model

## Notes

- The program runs each implementation 100 times for statistical significance
- Timing includes process startup time for Ruby
- AI comparison requires a running Ollama instance
- Falls back to simple comparison if Ollama is unavailable
