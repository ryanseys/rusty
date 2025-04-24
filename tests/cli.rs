use assert_cmd::Command;

#[test]
fn true_works() {
    assert!(true);
}

#[test]
fn ls_works() {
    let mut cmd = std::process::Command::new("ls");
    let result = cmd.output();
    assert!(result.is_ok());
}

#[test]
fn true_binary_returns_success() {
    let mut cmd = assert_cmd::Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_binary_returns_failure() {
    let mut cmd = assert_cmd::Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
