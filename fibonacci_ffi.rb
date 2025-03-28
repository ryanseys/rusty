require 'ffi'

module RustyLib
  extend FFI::Library
  # Look for the library in the target/release directory relative to this file
  lib_path = File.expand_path('target/release/librusty_lib', File.dirname(__FILE__))
  lib_path += '.dylib' if RbConfig::CONFIG['host_os'].include?('darwin')
  lib_path += '.so' if RbConfig::CONFIG['host_os'].include?('linux')
  lib_path += '.dll' if RbConfig::CONFIG['host_os'].include?('mingw')

  ffi_lib lib_path
  attach_function :fibonacci_ffi, [:uint], :uint64
end

def fibonacci_rust(n)
  RustyLib.fibonacci_ffi(n)
end

# Example usage:
if __FILE__ == $0
  n = ARGV[0].to_i
  result = fibonacci_rust(n)
  puts result
end
