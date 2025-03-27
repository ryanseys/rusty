require 'ffi'
require 'rbconfig'
require 'optparse'

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

def get_name_from_stdin
  print "Enter your name: "
  STDOUT.flush  # Ensure the prompt is displayed before reading
  gets.chomp
end

# Parse command line arguments
options = {}
OptionParser.new do |opts|
  opts.banner = "Usage: #{$0} [options]"
  opts.on("--name=NAME", "Your name") do |name|
    options[:name] = name
  end
end.parse!

# Get the name from command line or prompt
name = options[:name] || get_name_from_stdin

# Example numbers (you could add these as command line arguments too)
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
