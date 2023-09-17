//use std::process::Command;
use assert_cmd::Command;

#[test]
fn works(){
    assert!(true);
}

#[test]
fn runs(){
    //let mut cmd = Command::new("hello");
    //let res = cmd.output();
    let mut cmd = Command::cargo_bin("linux_cli").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");

    //assert!(res.is_ok());
}

#[test]
fn runs_ls(){
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}