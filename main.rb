require 'bundler/setup'
require 'optparse'
require 'ffi'
require 'json'

module Rusty
  extend FFI::Library

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

  def self.find_library
    paths = [
      File.expand_path("target/release/#{library_name}", __dir__),
      File.expand_path("target/debug/#{library_name}", __dir__)
    ]

    paths.find { |path| File.exist?(path) } or
      raise LoadError, "Could not find #{library_name} in:\n#{paths.join("\n")}"
  end

  def self.load_library!
    ffi_lib find_library

    # Define the functions we want to call
    attach_function :calculate_fibonacci_and_greet, [:string, :int], :pointer
    attach_function :calculate_fibonacci_batch, [:pointer, :int], :pointer
    attach_function :free_string, [:pointer], :void
  end

  def self.fibonacci_greeting(name, n)
    result_ptr = calculate_fibonacci_and_greet(name, n)
    result = result_ptr.read_string
    free_string(result_ptr)
    result
  end

  def self.fibonacci_batch(numbers)
    ptr = FFI::MemoryPointer.new(:int, numbers.size)
    ptr.write_array_of_int(numbers)

    result_ptr = calculate_fibonacci_batch(ptr, numbers.size)
    result = result_ptr.read_string
    free_string(result_ptr)

    JSON.parse(result)
  end
end

# Parse command line arguments
options = {
  name: 'friend',
  number: 10,
  rust: false,
  ruby: false,
  batch: false,
  batch_size: 10
}

OptionParser.new do |opts|
  opts.banner = "Usage: ruby main.rb [options]"

  opts.on("-n", "--name=NAME", "Name to greet") do |n|
    options[:name] = n
  end

  opts.on("-N", "--number=N", Integer, "Fibonacci number to calculate") do |n|
    options[:number] = n
  end

  opts.on("--rust", "Use Rust implementation") do
    options[:rust] = true
  end

  opts.on("--ruby", "Use Ruby implementation") do
    options[:ruby] = true
  end

  opts.on("--batch", "Run batch mode") do
    options[:batch] = true
  end

  opts.on("--batch-size=N", Integer, "Batch size for testing") do |n|
    options[:batch_size] = n
  end
end.parse!

# Default to Ruby if neither is specified
if !options[:rust] && !options[:ruby]
  options[:ruby] = true
end

def fibonacci_ruby(n)
  return 0 if n <= 0
  return 1 if n == 1

  a = 0
  b = 1
  (2..n).each do
    temp = a + b
    a = b
    b = temp
  end
  b
end

if options[:rust]
  Rusty.load_library!
  rust_internal_time = nil
  if options[:batch]
    # Generate a sequence of numbers for batch testing
    numbers = (1..options[:batch_size]).map { |i| options[:number] }
    start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    result = Rusty.fibonacci_batch(numbers)
    end_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    total_time = (end_time - start_time) * 1000
    rust_internal_time = result['time_ms']

    puts "Batch Results:"
    puts "Internal calculation time: #{result['time_ms']} ms"
    puts "Total time (including FFI): #{total_time.round(6)} ms"
    puts "Average time per calculation: #{(total_time / options[:batch_size]).round(6)} ms"
    puts "FFI overhead per call: #{((total_time - result['time_ms']) / options[:batch_size]).round(6)} ms"
  else
    start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    result = Rusty.fibonacci_greeting(options[:name], options[:number])
    end_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    total_time = (end_time - start_time) * 1000

    # Extract internal time from the result
    if result =~ /Rust Internal Time: ([\d.]+) milliseconds/
      rust_internal_time = $1.to_f
    end

    # Print the result (which includes the internal time on its own line)
    print result  # Using print because the result already includes newlines
    puts "Total time (including FFI): #{total_time.round(6)} ms"
  end
end

ruby_time = nil
if options[:ruby]
  if options[:batch]
    numbers = (1..options[:batch_size]).map { |i| options[:number] }
    start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    results = numbers.map { |n| fibonacci_ruby(n) }
    end_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    time_ms = (end_time - start_time) * 1000
    ruby_time = time_ms

    puts "\nRuby Batch Results:"
    puts "Total time: #{time_ms.round(6)} ms"
    puts "Average time per calculation: #{(time_ms / options[:batch_size]).round(6)} ms"
  else
    start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    result = fibonacci_ruby(options[:number])
    end_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
    time_ms = (end_time - start_time) * 1000
    ruby_time = time_ms

    puts "Hello #{options[:name]}! The #{options[:number]}th Fibonacci number is: #{result}"
    puts "Ruby Total Time: #{time_ms.round(6)} ms"
  end
end

# Compare Rust internal time with Ruby time if both were run
if rust_internal_time && ruby_time
  puts "\nComparison (Rust internal vs Ruby total):"
  if rust_internal_time < ruby_time
    speedup = ruby_time / rust_internal_time
    puts "Rust internal calculation was #{speedup.round(3)}x faster than Ruby"
  else
    speedup = rust_internal_time / ruby_time
    puts "Ruby was #{speedup.round(3)}x faster than Rust internal calculation"
  end
end
