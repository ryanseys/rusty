use std::process::Command;

#[test]
fn true_works() {
    assert!(true);
}

#[test]
fn ls_works() {
    let mut cmd = Command::new("ls");
    let result = cmd.output();
    assert!(result.is_ok());
}
