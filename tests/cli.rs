use assert_cmd::Command;

#[test]
fn works() {
    assert!(true);
    assert_ne!(false, true);
    assert_eq!(true, true);
    assert_eq!(1 + 1, 2);
}

#[test]
fn true_binary_returns_success() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_binary_returns_failure() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn hello_world_works() {
    let mut cmd = Command::cargo_bin("hello_world").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
