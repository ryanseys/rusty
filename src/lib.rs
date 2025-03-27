use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::time::Instant;

#[no_mangle]
pub extern "C" fn calculate_fibonacci_and_greet(name: *const c_char, n: c_int) -> *mut c_char {
    // Convert C string to Rust string
    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
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

#[no_mangle]
pub extern "C" fn calculate_fibonacci_batch(numbers: *const c_int, len: c_int) -> *mut c_char {
    let numbers = unsafe { std::slice::from_raw_parts(numbers, len as usize) };

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

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
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
