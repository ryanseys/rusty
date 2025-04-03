use rusty::divide::divide;

#[test]
fn test_divide_success() {
    assert_eq!(divide(10.0, 2.0), Ok(5.0));
}

#[test]
fn test_divide_by_zero_returns_error() {
    assert_eq!(
        divide(10.0, 0.0),
        Err("Error: Cannot divide by zero".to_string())
    );
}
