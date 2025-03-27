# Rusty

A proof of concept project demonstrating Ruby and Rust integration using FFI (Foreign Function Interface) for calculating Fibonacci numbers.

## Features

- Calculate Fibonacci numbers using either Rust or Ruby implementations
- Measure and compare performance between implementations
- Detailed timing information including:
  - Rust internal calculation time
  - Total execution time (including FFI overhead for Rust)
  - Performance comparison between implementations
- Support for batch processing to analyze FFI overhead
- Native Rust command-line interface

## Quick Start

1. Make sure you have Rust and Ruby installed
2. Clone this repository
3. Build and run:
```bash
# Build the project
cargo build --release

# Run the command
./rusty
```

## Usage

The `rusty` command is a native Rust binary that supports several options:

```bash
./rusty [options]

Options:
  -n, --name NAME     Set the name for greeting (default: friend)
  -N, --number N      Set the Fibonacci number to calculate (default: 10)
  --rust             Use Rust implementation
  --ruby             Use Ruby implementation
  --batch            Run in batch mode for performance analysis
  --batch-size N     Set batch size for testing (default: 10)
  -h, --help         Show this help message
```

### Example Commands

1. Run both implementations and compare performance:
```bash
./rusty --rust --ruby -N 100
```

2. Run just the Rust implementation:
```bash
./rusty --rust -N 100
```

3. Run just the Ruby implementation:
```bash
./rusty --ruby -N 100
```

4. Run batch mode for performance analysis:
```bash
./rusty --rust --ruby --batch -N 100
```

5. Customize the greeting:
```bash
./rusty --rust --ruby -n "Alice" -N 50
```

### Output Example

When running both implementations, you'll see output like this:

```
Hello friend! The 100th Fibonacci number is: 354224848179261915075
Rust Internal Time: 0.001234 milliseconds
Total time (including FFI): 266.639 ms

Hello friend! The 100th Fibonacci number is: 354224848179261915075
Ruby Total Time: 0.123456 ms

Comparison (Rust internal vs Ruby total):
Rust internal calculation was 100.047x faster than Ruby
```

## Implementation Details

### Command Line Interface
- Written in Rust using `clap` for argument parsing
- Manages building and running both implementations
- Provides colored output and detailed timing information
- Handles process management and error reporting

### Rust Implementation
- Located in `src/lib.rs`
- Uses `u128` for large Fibonacci numbers
- Maximum supported Fibonacci number is 184 (largest that fits in u128)
- Includes internal timing measurements
- Compiled as a dynamic library for Ruby FFI

### Ruby Implementation
- Located in `main.rb`
- Pure Ruby implementation for comparison
- Uses FFI gem to interface with Rust
- Includes timing measurements
- Supports both single and batch calculations

### Performance Considerations
- Rust internal calculation is typically faster
- FFI overhead can be significant for single calculations
- Batch mode available to analyze FFI overhead
- Performance comparison shows actual calculation speed difference

## Project Structure

```
.
├── rusty             # Native Rust command-line interface
├── src/
│   ├── lib.rs       # Rust Fibonacci implementation
│   └── main.rs      # Rust CLI implementation
├── main.rb          # Ruby implementation
├── Cargo.toml       # Rust dependencies and build configuration
├── Gemfile         # Ruby dependencies
└── README.md       # This file
```

## Dependencies

### Rust
- `libc` for FFI compatibility
- `clap` for command-line argument parsing
- `colored` for terminal output formatting
- `serde_json` for JSON handling

### Ruby
- `ffi` gem for Rust integration
- `bundler` for dependency management

## Development

To modify the project:

1. Command-line interface changes:
   - Edit `src/main.rs`
   - Run `cargo build --release` to rebuild

2. Rust implementation changes:
   - Edit `src/lib.rs`
   - Project will automatically rebuild when running `./rusty`

3. Ruby implementation changes:
   - Edit `main.rb`
   - No build step needed

## Notes

- The Rust implementation is limited to the 184th Fibonacci number due to u128 constraints
- FFI overhead is more noticeable for small numbers
- Batch mode is useful for performance analysis
- Internal timing measurements help distinguish between calculation time and FFI overhead
- The command-line interface is now a native Rust binary for better performance and error handling