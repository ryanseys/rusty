# frozen_string_literal: true
require_relative 'fibonacci_ffi'
require 'fiddle'
require 'fiddle/import'

def fibonacci(n)
  return n if n <= 1

  a = 0
  b = 1
  (n - 1).times do
    a, b = b, a + b
  end
  b
end

n = ARGV[0].to_i
result = fibonacci(n)
rust_result = fibonacci_rust(n)
puts result

module RubyExports
  extend Fiddle::Importer

  # Export Ruby functions for Rust to call
  def self.fibonacci(n)
    return n if n <= 1
    a, b = 0, 1
    (2..n).each do
      a, b = b, a + b
    end
    b
  end

  # Create a C-callable function
  @fibonacci_callback = Fiddle::Closure::BlockCaller.new(
    Fiddle::TYPE_LONG_LONG, # return type
    [Fiddle::TYPE_INT]      # parameter types
  ) do |n|
    fibonacci(n)
  end

  # Export the function pointer address for Rust to use
  def self.get_fibonacci_fn_ptr
    @fibonacci_callback.to_i
  end
end

# Make the function pointer globally accessible
$fibonacci_fn_ptr = RubyExports.get_fibonacci_fn_ptr
