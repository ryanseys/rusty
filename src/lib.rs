pub mod divide;
use libc::{c_int, c_longlong, c_uint};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::time::Instant;

// Type definition for Ruby function pointer
type RubyFibonacciFn = unsafe extern "C" fn(c_int) -> c_longlong;

/// Calculates a Fibonacci number and creates a greeting message.
///
/// # Safety
///
/// The caller must ensure:
/// * `name` is a valid pointer to a null-terminated C string
/// * `name` points to valid UTF-8 data
/// * `name` is valid for reads
/// * `name` is not null (this function will assert on null)
///
/// Returns a pointer to a newly allocated C string that must be freed using `free_string`.
#[no_mangle]
pub unsafe extern "C" fn calculate_fibonacci_and_greet(
    name: *const c_char,
    n: c_int,
) -> *mut c_char {
    // Convert C string to Rust string
    assert!(!name.is_null());
    let name = CStr::from_ptr(name);
    let name_str = name.to_str().unwrap_or("friend");

    // Calculate fibonacci with timing
    let start = Instant::now();
    let result = fibonacci(n);
    let duration = start.elapsed();
    let time_ms = duration.as_secs_f64() * 1000.0;

    // Create the greeting with the fibonacci result and timing
    let result = format!(
        "Hello {}! The {}th Fibonacci number is: {}\nRust Internal Time: {:.6} milliseconds\n",
        name_str, n, result, time_ms
    );

    // Convert the result back to a C string
    let c_result = CString::new(result).unwrap();
    c_result.into_raw() // Transfer ownership to Ruby
}

/// Calculates Fibonacci numbers for a batch of inputs.
///
/// # Safety
///
/// The caller must ensure:
/// * `numbers` is a valid pointer to an array of `len` consecutive `c_int` values
/// * `numbers` is valid for reads of `len * size_of::<c_int>()` bytes
/// * `numbers` is properly aligned for `c_int`
/// * `len` accurately represents the length of the array
/// * The memory referenced by `numbers` is not mutated during this function's execution
///
/// Returns a pointer to a newly allocated C string that must be freed using `free_string`.
#[no_mangle]
pub unsafe extern "C" fn calculate_fibonacci_batch(
    numbers: *const c_int,
    len: c_int,
) -> *mut c_char {
    let numbers = std::slice::from_raw_parts(numbers, len as usize);

    let start = Instant::now();
    let mut results = Vec::with_capacity(len as usize);

    for &n in numbers {
        results.push(fibonacci(n));
    }

    let duration = start.elapsed();
    let time_ms = duration.as_secs_f64() * 1000.0;

    // Format results as JSON-like string
    let result = format!(
        "{{\"results\": [{:?}], \"time_ms\": {:.6}}}",
        results, time_ms
    );

    let c_result = CString::new(result).unwrap();
    c_result.into_raw()
}

/// Frees a C string previously allocated by Rust.
///
/// # Safety
///
/// The caller must ensure:
/// * `ptr` was previously returned by `calculate_fibonacci_and_greet` or `calculate_fibonacci_batch`
/// * `ptr` has not been freed before
/// * `ptr` will not be used after this call
/// * If `ptr` is null, this function is a no-op
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

fn fibonacci(n: i32) -> u128 {
    // The 184th Fibonacci number is the largest that fits in u128
    if n > 184 {
        eprintln!(
            "Warning: n={} is too large (max=184 for u128). Using n=184.",
            n
        );
        return fibonacci(184);
    }

    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0u128;
    let mut b = 1u128;

    for _ in 2..=n {
        let temp = a.checked_add(b).unwrap_or_else(|| {
            eprintln!("Warning: Fibonacci number overflow, returning max value");
            u128::MAX
        });
        a = b;
        b = temp;
    }

    b
}

#[no_mangle]
pub extern "C" fn fibonacci_ffi(n: c_uint) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// Calls a Ruby function pointer to calculate Fibonacci numbers.
///
/// # Safety
///
/// The caller must ensure:
/// * `ruby_fn_ptr` is a valid function pointer to a Ruby function matching the `RubyFibonacciFn` type
/// * The Ruby function is still valid and callable
/// * The Ruby runtime is initialized and ready to execute the function
#[no_mangle]
pub unsafe extern "C" fn call_ruby_fibonacci(ruby_fn_ptr: usize, n: c_int) -> c_longlong {
    let ruby_fn: RubyFibonacciFn = unsafe { mem::transmute(ruby_fn_ptr) };
    unsafe { ruby_fn(n) }
}

// Function to benchmark Ruby calls
pub fn benchmark_ruby_ffi(ruby_fn_ptr: usize, n: u32) -> (u64, Vec<f64>) {
    let mut times = Vec::with_capacity(100);
    let mut result = 0;

    for _ in 0..100 {
        let start = std::time::Instant::now();
        result = unsafe { call_ruby_fibonacci(ruby_fn_ptr, n as c_int) } as u64;
        let duration = start.elapsed().as_secs_f64() * 1000.0;
        times.push(duration);
    }

    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (result, times)
}
