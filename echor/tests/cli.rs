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
  run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
  run(&["Hello there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
  run(&["Hello  there","-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
  run(&["Hello there","-n"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str],expected_file: &str) -> TestResult {
  let expected = fs::read_to_string(expected_file)?;
  Command::cargo_bin("echor")?
    .args(args)
    .assert()
    .success()
    .stdout(expected);
  Ok(())

}
