# frozen_string_literal: true

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
puts fibonacci(n)
