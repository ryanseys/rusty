#!/usr/bin/env ruby
require 'json'
require 'benchmark'

# Configuration
FIBONACCI_NUMBERS = [1, 5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 120, 140, 160, 180]
ITERATIONS = 3  # Number of times to run each test for averaging

def run_rust_fibonacci(n)
  output = `ruby main.rb --number=#{n} --rust`
  if output =~ /internal_time: ([\d.]+) ms/
    internal_time = $1.to_f
    return {
      total_time: nil,  # Will be measured separately
      internal_time: internal_time
    }
  end
  nil
end

def run_ruby_fibonacci(n)
  start_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  output = `ruby main.rb --number=#{n} --ruby`
  end_time = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  internal_time = (end_time - start_time) * 1000  # Convert to ms

  if output =~ /number is: (\d+)/
    return {
      total_time: nil,  # Will be measured separately
      internal_time: internal_time
    }
  end
  nil
end

def measure_total_time(implementation, n)
  output = `ruby benchmark_helper.rb --number=#{n} --#{implementation}`
  if output =~ /TIME:([\d.]+)/
    $1.to_f
  else
    nil
  end
end

def run_simulation
  results = {
    fibonacci_numbers: FIBONACCI_NUMBERS,
    rust_results: [],
    ruby_results: []
  }

  FIBONACCI_NUMBERS.each do |n|
    puts "Testing Fibonacci(#{n})..."

    # Initialize accumulators
    rust_total = 0
    rust_internal = 0
    ruby_total = 0
    ruby_internal = 0

    ITERATIONS.times do |i|
      # Rust measurements
      rust_result = run_rust_fibonacci(n)
      rust_total_time = measure_total_time('rust', n)

      rust_total += rust_total_time if rust_total_time
      rust_internal += rust_result[:internal_time] if rust_result

      # Ruby measurements
      ruby_result = run_ruby_fibonacci(n)
      ruby_total_time = measure_total_time('ruby', n)

      ruby_total += ruby_total_time if ruby_total_time
      ruby_internal += ruby_result[:internal_time] if ruby_result
    end

    # Calculate averages
    results[:rust_results] << {
      total_time: rust_total / ITERATIONS,
      internal_time: rust_internal / ITERATIONS
    }

    results[:ruby_results] << {
      total_time: ruby_total / ITERATIONS,
      internal_time: ruby_internal / ITERATIONS
    }
  end

  results
end

# Run simulation and save results
results = run_simulation
File.write('simulation_results.json', JSON.pretty_generate(results))
puts "Results saved to simulation_results.json"
