# Rusty

I'm learning Rust... I'm still pretty rusty. This might be a mess, but full of vibes 👽

This is a minimal proof of concept demonstrating how to call Rust code from Ruby using FFI (Foreign Function Interface). The example shows how to:
- Pass strings and integers from Ruby to Rust
- Perform calculations in Rust
- Return strings from Rust to Ruby
- Handle memory management properly between languages
- Handle command line arguments and interactive input

## Prerequisites

- Rust (latest stable version)
  ```bash
  # Install Rust if you haven't already
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Ruby (3.x)
  ```bash
  # Check your Ruby version
  ruby --version
  ```

- Bundler (for managing Ruby dependencies)
  ```bash
  gem install bundler
  ```

## Project Structure

```
.
├── Cargo.toml          # Rust project configuration
├── Gemfile            # Ruby dependencies
├── Gemfile.lock       # Locked Ruby dependency versions
├── src/
│   └── lib.rs         # Rust FFI library code (Rust FFI functions)
└── main.rb            # Ruby script that calls Rust code
```

## Setup and Installation

1. Clone the repository and navigate to the project directory:
   ```bash
   git clone <repository-url>
   cd <project-directory>
   ```

2. Install Ruby dependencies:
   ```bash
   bundle install
   ```

   If you encounter any issues with the FFI gem, try reinstalling it with native extensions:
   ```bash
   gem uninstall ffi
   gem install ffi --platform=ruby
   bundle install
   ```

3. Build the Rust library:
   ```bash
   cargo build --release
   ```
   This will create a dynamic library in the `target/release` directory:
   - macOS: `librusty.dylib`
   - Linux: `librusty.so`
   - Windows: `rusty.dll`

## Running the Example

You can run the script in two ways:

1. With a command line argument:
   ```bash
   bundle exec ruby main.rb --name=Ryan
   ```

2. Interactively (will prompt for name):
   ```bash
   bundle exec ruby main.rb
   ```

You should see output like:
```
# When using --name=Ryan:
Hello Ryan! The sum is: 8

# When running interactively:
Enter your name: Ryan
Hello Ryan! The sum is: 8
```

## How It Works

The example demonstrates three key components working together:

1. **Rust Library (src/lib.rs)**
   - Defines FFI-compatible functions that can be called from Ruby
   - Handles string conversions between Rust and C
   - Manages memory safely
   - Performs the actual computation (adding numbers and creating a greeting)

2. **Ruby Script (main.rb)**
   - Uses the FFI gem to load the Rust dynamic library
   - Provides a clean Ruby interface to the Rust functions
   - Handles proper memory management
   - Includes cross-platform library path detection
   - Implements error handling
   - Processes command line arguments using `optparse`
   - Falls back to interactive STDIN input if no name is provided

3. **Build Configuration**
   - Cargo.toml: Configures the Rust project to build as a dynamic library
   - Gemfile: Manages Ruby dependencies (primarily FFI)

## Common Issues and Solutions

1. **Library Loading Issues**
   - Ensure the Rust library is built (`cargo build --release`)
   - Check that the library file exists in `target/release/`
   - The script automatically checks both release and debug builds

2. **FFI Gem Issues**
   - If you see FFI-related errors, try reinstalling the gem:
     ```bash
     gem uninstall ffi
     gem install ffi --platform=ruby
     bundle install
     ```

3. **Platform-Specific Issues**
   - The code automatically detects your OS and uses the appropriate library extension
   - Supported platforms: macOS (.dylib), Linux (.so), Windows (.dll)

## Development

To modify the example:

1. Edit the Rust code in `src/lib.rs`
2. Rebuild the library:
   ```bash
   cargo build --release
   ```
3. Edit the Ruby code in `main.rb`
4. Run with:
   ```bash
   bundle exec ruby main.rb
   ```

Remember to rebuild the Rust library after any changes to the Rust code.

## Memory Management

The example demonstrates proper memory management between Ruby and Rust:
- Rust allocates memory for the return string
- Ruby receives a pointer to this memory
- Ruby reads the string and explicitly frees the memory using the provided `free_string` function
- The FFI boundary is handled safely to prevent memory leaks

## Contributing

Feel free to submit issues and enhancement requests!