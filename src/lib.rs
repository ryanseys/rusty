use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn add_numbers_and_greet(name: *const c_char, a: c_int, b: c_int) -> *mut c_char {
    // Convert C string to Rust string
    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name_str = name.to_str().unwrap_or("friend");

    // Perform the addition
    let sum = a + b;

    // Create the greeting
    let result = format!("Hello {}! The sum is: {}", name_str, sum);

    // Convert the result back to a C string
    let c_result = CString::new(result).unwrap();
    c_result.into_raw() // Transfer ownership to Ruby
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
