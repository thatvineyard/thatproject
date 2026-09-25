use assert_cmd::Command;

struct TestCase {
    name: &'static str,
    args: &'static [&'static str],
}

const TEST_CASES: &[TestCase] = &[
  TestCase { name: "greet", args: &[]},
  TestCase { name: "greet", args: &["--name", "testname"]},
];

#[test]
fn test_running_commands() {
  for case in TEST_CASES {
    let mut command = Command::cargo_bin("thatproject").unwrap();
    command.arg(case.name).args(case.args);
    command.assert().success();
  }
}