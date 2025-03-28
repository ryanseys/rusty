# Changelog

## [Unreleased]

### Added

- Added comprehensive benchmarking with 100 iterations for both Rust and Ruby implementations
- Added statistical analysis including mean, median, P95, min, and max timings
- Added AI-powered performance comparison using local Ollama instance
- Added colorful terminal output with tables and emojis
- Added default Fibonacci number calculation of N=88
- Implemented bi-directional FFI between Rust and Ruby
- Added Rust→Ruby FFI benchmarking
- Extended comparison table to show both FFI directions

### Changed

- Refactored Ruby implementation for better performance using iterative approach
- Simplified Ruby code by removing unnecessary module structure
- Updated performance comparison table to show consistent "Median Time" labels
- Added Fibonacci number (N) to results table for clarity
- Improved output formatting with emojis (🦀 for Rust, 💎 for Ruby)

### Removed

- Removed greeting functionality from both Rust and Ruby implementations
- Removed unnecessary documentation and comments
- Removed FFI integration in favor of separate process comparison

### Fixed

- Fixed Ruby result parsing and display in comparison table
- Fixed inconsistent timing labels in output
