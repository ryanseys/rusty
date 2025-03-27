require 'ffi'
require 'rbconfig'

module Rusty
  extend FFI::Library

  # Determine the library extension based on the OS
  def self.library_name
    case RbConfig::CONFIG['host_os']
    when /darwin/
      'librusty.dylib'
    when /linux/
      'librusty.so'
    when /mswin|mingw/
      'rusty.dll'
    else
      raise "Unsupported platform"
    end
  end

  # Try multiple possible library paths
  def self.find_library
    paths = [
      File.expand_path("target/release/#{library_name}", __dir__),
      File.expand_path("target/debug/#{library_name}", __dir__)
    ]

    paths.find { |path| File.exist?(path) } or
      raise LoadError, "Could not find #{library_name} in:\n#{paths.join("\n")}"
  end

  # Load the dynamic library
  ffi_lib find_library

  # Define the functions we want to call
  attach_function :add_numbers_and_greet, [:string, :int, :int], :pointer
  attach_function :free_string, [:pointer], :void

  def self.add_and_greet(name, a, b)
    # Call the Rust function
    result_ptr = add_numbers_and_greet(name, a, b)

    # Convert the result to a Ruby string
    result = result_ptr.read_string

    # Free the memory allocated by Rust
    free_string(result_ptr)

    result
  end
end

# Example usage
name = "Ryan"
num1 = 5
num2 = 3

begin
  result = Rusty.add_and_greet(name, num1, num2)
  puts result
rescue => e
  puts "Error: #{e.message}"
  puts "Backtrace:"
  puts e.backtrace
end
