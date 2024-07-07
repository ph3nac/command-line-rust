use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

type TestResult = Result<(),Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult{
  Command::cargo_bin("echor")?
    .assert()
    .failure()
    .stderr(predicate::str::contains("USAGE"));
  Ok(())
}

#[test]
fn runs() -> TestResult{
  Command::cargo_bin("echor")?
    .arg("hello").assert().success();
  Ok(())
}

#[test]
fn hello1() -> TestResult{
  let expected = fs::read_to_string("tests/expected/hello1.txt")?;
  let mut cmd = Command::cargo_bin("echor")?;
  cmd.arg("Hello there").assert().success().stdout(expected);
  Ok(())
}
