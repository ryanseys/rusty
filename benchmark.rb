#!/usr/bin/env ruby
# frozen_string_literal: true

require 'benchmark'
require_relative 'main'
require_relative 'fibonacci'

# Default configuration
ITERATIONS = 1000
TEST_NUMBERS = [10, 20, 40, 80, 100]
NAME = "Benchmark"

def format_time(time)
  case time
  when 0..0.000001 then "#{(time * 1_000_000).round(2)}ns"
  when 0.000001..0.001 then "#{(time * 1_000).round(2)}µs"
  when 0.001..1 then "#{(time * 1_000).round(2)}ms"
  else "#{time.round(2)}s"
  end
end

def print_config_message
  puts "\nBenchmark Configuration:"
  puts "- Test numbers: #{TEST_NUMBERS.join(', ')}"
  puts "- Iterations per test: #{ITERATIONS}"
  puts "- Test name: #{NAME}"
  puts "\nTo modify these values, edit the constants at the top of benchmark.rb"
  puts "=" * 60
end

def print_header
  puts "\nFibonacci Performance Comparison (#{ITERATIONS} iterations each)"
  puts "=" * 60
  puts sprintf("%-6s %-15s %-15s %-15s", "n", "Ruby Avg", "Rust Avg", "Rust Speedup")
  puts "-" * 60
end

def print_result(n, ruby_avg, rust_avg, speedup)
  puts sprintf("%-6d %-15s %-15s %-15s",
    n,
    format_time(ruby_avg),
    format_time(rust_avg),
    "#{speedup.round(2)}x"
  )
end

def run_benchmark
  # Show configuration
  print_config_message

  # Initialize Rust library
  Rusty.load_library!

  results = []

  print_header

  TEST_NUMBERS.each do |n|
    print "Benchmarking Fibonacci(#{n})... "

    # Warmup
    FibonacciCalculator.fibonacci(n)
    Rusty.fibonacci_greeting(NAME, n)

    # Benchmark Ruby
    ruby_time = Benchmark.realtime do
      ITERATIONS.times { FibonacciCalculator.fibonacci(n) }
    end

    # Benchmark Rust
    rust_time = Benchmark.realtime do
      ITERATIONS.times { Rusty.fibonacci_greeting(NAME, n) }
    end

    # Calculate average times
    ruby_avg = ruby_time / ITERATIONS
    rust_avg = rust_time / ITERATIONS

    # Calculate speedup
    speedup = ruby_avg / rust_avg

    print_result(n, ruby_avg, rust_avg, speedup)
    results << speedup

    puts "Done!"
  end

  puts "\nSummary:"
  puts "- Test iterations: #{ITERATIONS}"
  puts "- Maximum Rust speedup: #{results.max.round(2)}x"
  puts "- Ruby implementation: Pure Ruby"
  puts "- Rust implementation: Native code via FFI"
  puts "\nNote: Lower numbers are better. Speedup > 1 means Rust is faster."
end

begin
  run_benchmark
rescue => e
  puts "Error during benchmark: #{e.message}"
  puts e.backtrace
  exit 1
end
