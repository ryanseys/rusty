# frozen_string_literal: true

module FibonacciCalculator
  # Calculates the nth Fibonacci number using an efficient iterative approach
  # @param n [Integer] the position in the Fibonacci sequence (0-based)
  # @return [Integer] the nth Fibonacci number
  def self.fibonacci(n)
    return 0 if n <= 0
    return 1 if n == 1

    a, b = 0, 1
    (2..n).each do
      a, b = b, a + b
    end
    b
  end

  # Generates a greeting with the calculated Fibonacci number
  # @param name [String] the name to include in the greeting
  # @param n [Integer] the position in the Fibonacci sequence
  # @return [String] the formatted greeting
  def self.fibonacci_greeting(name, n)
    fib_result = fibonacci(n)
    "Hello #{name}! The #{n}th Fibonacci number is: #{fib_result}"
  end
end
